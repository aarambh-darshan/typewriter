//! # typewriter-plugin
//!
//! Plugin API for the typewriter type sync SDK.
//!
//! This crate defines the contract that external language emitter plugins must implement.
//! Plugin authors depend on this crate, implement the [`EmitterPlugin`] trait, and use
//! the [`declare_plugin!`] macro to expose C ABI entry points for dynamic loading.
//!
//! ## Writing a Plugin
//!
//! ```rust,ignore
//! use typewriter_plugin::prelude::*;
//!
//! struct MyMapper;
//!
//! impl TypeMapper for MyMapper {
//!     fn map_primitive(&self, ty: &PrimitiveType) -> String { todo!() }
//!     fn map_option(&self, inner: &TypeKind) -> String { todo!() }
//!     fn map_vec(&self, inner: &TypeKind) -> String { todo!() }
//!     fn map_hashmap(&self, key: &TypeKind, value: &TypeKind) -> String { todo!() }
//!     fn map_tuple(&self, elements: &[TypeKind]) -> String { todo!() }
//!     fn map_named(&self, name: &str) -> String { todo!() }
//!     fn emit_struct(&self, def: &StructDef) -> String { todo!() }
//!     fn emit_enum(&self, def: &EnumDef) -> String { todo!() }
//!     fn file_header(&self, type_name: &str) -> String { todo!() }
//!     fn file_extension(&self) -> &str { todo!() }
//!     fn file_naming(&self, type_name: &str) -> String { todo!() }
//! }
//!
//! struct MyPlugin;
//!
//! impl EmitterPlugin for MyPlugin {
//!     fn language_id(&self) -> &str { "mylang" }
//!     fn language_name(&self) -> &str { "My Language" }
//!     fn version(&self) -> &str { "0.1.0" }
//!     fn default_output_dir(&self) -> &str { "./generated/mylang" }
//!     fn mapper(&self, _config: &PluginConfig) -> Box<dyn TypeMapper> {
//!         Box::new(MyMapper)
//!     }
//! }
//!
//! declare_plugin!(MyPlugin);
//! ```

use serde::Deserialize;

// Re-export everything plugin authors need
pub use typewriter_core::ir::*;
pub use typewriter_core::mapper::TypeMapper;
pub use typewriter_core::naming::{FileStyle, to_file_style};

/// Current plugin API version.
///
/// Plugins built against a different API version will be rejected at load time.
/// This is bumped whenever the `EmitterPlugin` or `TypeMapper` trait changes
/// in a backward-incompatible way.
pub const PLUGIN_API_VERSION: u32 = 1;

/// Configuration data passed to a plugin from `typewriter.toml`.
///
/// This contains the plugin-specific TOML section (e.g. `[ruby]`) parsed into
/// a generic structure. Each plugin can define its own config keys.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct PluginConfig {
    /// Output directory override from config
    pub output_dir: Option<String>,
    /// File naming style override from config
    pub file_style: Option<String>,
    /// All extra key-value pairs from the plugin's TOML section
    #[serde(flatten)]
    pub extra: toml::Table,
}

/// The core trait that every plugin must implement.
///
/// This defines the contract between typewriter and external language emitter plugins.
/// Each plugin provides metadata about the language it targets and a factory method
/// for creating the actual `TypeMapper` implementation.
pub trait EmitterPlugin: Send + Sync {
    /// Unique identifier for this language (e.g. `"ruby"`, `"php"`, `"dart"`).
    ///
    /// This is used in `#[sync_to(ruby)]` and as the TOML section key `[ruby]`.
    fn language_id(&self) -> &str;

    /// Human-readable display name (e.g. `"Ruby"`, `"PHP"`, `"Dart/Flutter"`).
    fn language_name(&self) -> &str;

    /// Plugin version as a semver string (e.g. `"0.1.0"`).
    fn version(&self) -> &str;

    /// Default output directory when not configured in `typewriter.toml`.
    fn default_output_dir(&self) -> &str;

    /// Create a `TypeMapper` instance for this language.
    ///
    /// The returned mapper is used to render type definitions.
    /// The `config` parameter contains plugin-specific settings from `typewriter.toml`.
    fn mapper(&self, config: &PluginConfig) -> Box<dyn TypeMapper>;

    /// The TOML section key for this plugin's configuration.
    ///
    /// Defaults to `language_id()`. Override if you want a different key.
    fn config_key(&self) -> &str {
        self.language_id()
    }

    /// File extension for generated files (without leading dot).
    ///
    /// Used by drift detection and reporting. Defaults to asking the mapper,
    /// but can be overridden for cases where the mapper isn't available yet.
    fn file_extension(&self) -> &str;

    /// Plugin API version this plugin was built against.
    ///
    /// Do not override — this is checked at load time for compatibility.
    fn api_version(&self) -> u32 {
        PLUGIN_API_VERSION
    }
}

/// Declare a plugin's C ABI entry points for dynamic loading.
///
/// This macro generates the `extern "C"` functions that `typewriter-engine`
/// calls when loading a plugin from a shared library (`.so`/`.dylib`/`.dll`).
///
/// # Usage
///
/// ```rust,ignore
/// declare_plugin!(MyPlugin);
/// ```
///
/// This expands to:
/// - `_tw_plugin_create()` → returns a raw pointer to a boxed `EmitterPlugin`
/// - `_tw_plugin_api_version()` → returns the API version constant
#[macro_export]
macro_rules! declare_plugin {
    ($plugin_type:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn _tw_plugin_create() -> *mut dyn $crate::EmitterPlugin {
            let plugin: Box<dyn $crate::EmitterPlugin> = Box::new(<$plugin_type>::new());
            Box::into_raw(plugin)
        }

        #[unsafe(no_mangle)]
        pub extern "C" fn _tw_plugin_api_version() -> u32 {
            $crate::PLUGIN_API_VERSION
        }
    };
}

/// Convenience prelude for plugin authors.
///
/// ```rust,ignore
/// use typewriter_plugin::prelude::*;
/// ```
pub mod prelude {
    pub use super::{
        EmitterPlugin, PLUGIN_API_VERSION, PluginConfig,
    };
    pub use typewriter_core::ir::*;
    pub use typewriter_core::mapper::TypeMapper;
    pub use typewriter_core::naming::{FileStyle, to_file_style};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_api_version() {
        assert_eq!(PLUGIN_API_VERSION, 1);
    }

    #[test]
    fn test_plugin_config_default() {
        let config = PluginConfig::default();
        assert!(config.output_dir.is_none());
        assert!(config.file_style.is_none());
        assert!(config.extra.is_empty());
    }

    #[test]
    fn test_plugin_config_deserialization() {
        let toml_str = r#"
output_dir = "./generated/ruby"
file_style = "snake_case"
gem_version = "3.2"
"#;
        let config: PluginConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.output_dir.as_deref(), Some("./generated/ruby"));
        assert_eq!(config.file_style.as_deref(), Some("snake_case"));
        assert_eq!(
            config.extra.get("gem_version").and_then(|v| v.as_str()),
            Some("3.2")
        );
    }
}
