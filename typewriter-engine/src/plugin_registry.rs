//! Plugin registry: dynamic loading and management of external emitter plugins.

use anyhow::{Context, Result, bail};
use std::path::{Path, PathBuf};
use typewriter_plugin::{EmitterPlugin, PLUGIN_API_VERSION, PluginConfig};

use crate::emit::GeneratedFile;
use typewriter_core::ir::TypeDef;

/// Information about a loaded plugin.
#[derive(Debug, Clone)]
pub struct PluginInfo {
    /// Language identifier (e.g. "ruby")
    pub language_id: String,
    /// Human-readable name (e.g. "Ruby (Sorbet)")
    pub language_name: String,
    /// Plugin version
    pub version: String,
    /// Default output directory
    pub default_output_dir: String,
    /// File extension
    pub file_extension: String,
    /// Path the plugin was loaded from (if dynamic)
    pub source_path: Option<PathBuf>,
}

/// A loaded plugin with its associated dynamic library handle.
struct LoadedPlugin {
    plugin: Box<dyn EmitterPlugin>,
    _lib: Option<libloading::Library>,
    source_path: Option<PathBuf>,
}

/// Registry for managing loaded emitter plugins.
///
/// The registry discovers, loads, and manages external language emitter plugins.
/// Plugins are loaded as shared libraries (`.so`/`.dylib`/`.dll`) at runtime.
pub struct PluginRegistry {
    plugins: Vec<LoadedPlugin>,
}

impl PluginRegistry {
    /// Create an empty plugin registry.
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    /// Load all plugins from a directory.
    ///
    /// Scans the directory for shared library files (`.so`, `.dylib`, `.dll`)
    /// and attempts to load each one as a typewriter plugin.
    pub fn load_from_dir(&mut self, dir: &Path) -> Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        let entries = std::fs::read_dir(dir)
            .with_context(|| format!("failed to read plugin directory: {}", dir.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && is_shared_library(&path) {
                match self.load_plugin(&path) {
                    Ok(()) => {}
                    Err(err) => {
                        eprintln!(
                            "typewriter: warning: failed to load plugin {}: {}",
                            path.display(),
                            err
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Load a single plugin from a shared library path.
    ///
    /// Verifies the API version and registers the plugin.
    pub fn load_plugin(&mut self, path: &Path) -> Result<()> {
        // Expand ~ in paths
        let expanded = expand_tilde(path);

        unsafe {
            let lib = libloading::Library::new(&expanded)
                .with_context(|| format!("failed to load shared library: {}", expanded.display()))?;

            // Check API version first
            let api_version_fn: libloading::Symbol<unsafe extern "C" fn() -> u32> = lib
                .get(b"_tw_plugin_api_version")
                .with_context(|| {
                    format!(
                        "shared library {} is not a valid typewriter plugin (missing _tw_plugin_api_version)",
                        expanded.display()
                    )
                })?;

            let plugin_api_version = api_version_fn();
            if plugin_api_version != PLUGIN_API_VERSION {
                bail!(
                    "plugin {} has API version {} but typewriter requires version {}. \
                     Please rebuild the plugin against the current typewriter-plugin crate.",
                    expanded.display(),
                    plugin_api_version,
                    PLUGIN_API_VERSION
                );
            }

            // Create the plugin instance
            let create_fn: libloading::Symbol<unsafe extern "C" fn() -> *mut dyn EmitterPlugin> = lib
                .get(b"_tw_plugin_create")
                .with_context(|| {
                    format!(
                        "shared library {} is not a valid typewriter plugin (missing _tw_plugin_create)",
                        expanded.display()
                    )
                })?;

            let plugin_ptr = create_fn();
            if plugin_ptr.is_null() {
                bail!("plugin {} returned null from _tw_plugin_create", expanded.display());
            }

            let plugin = Box::from_raw(plugin_ptr);

            // Check for duplicate language_id
            let lang_id = plugin.language_id().to_string();
            if self.get(&lang_id).is_some() {
                bail!(
                    "plugin {} provides language '{}' which is already registered",
                    expanded.display(),
                    lang_id
                );
            }

            eprintln!(
                "  typewriter: loaded plugin '{}' v{} ({})",
                plugin.language_name(),
                plugin.version(),
                expanded.display()
            );

            self.plugins.push(LoadedPlugin {
                plugin,
                _lib: Some(lib),
                source_path: Some(expanded),
            });
        }

        Ok(())
    }

    /// Register a plugin directly (for testing or static registration).
    pub fn register(&mut self, plugin: Box<dyn EmitterPlugin>) {
        self.plugins.push(LoadedPlugin {
            plugin,
            _lib: None,
            source_path: None,
        });
    }

    /// Get a plugin by its language ID.
    pub fn get(&self, language_id: &str) -> Option<&dyn EmitterPlugin> {
        self.plugins
            .iter()
            .find(|lp| lp.plugin.language_id() == language_id)
            .map(|lp| lp.plugin.as_ref())
    }

    /// List all loaded plugins.
    pub fn list(&self) -> Vec<PluginInfo> {
        self.plugins
            .iter()
            .map(|lp| PluginInfo {
                language_id: lp.plugin.language_id().to_string(),
                language_name: lp.plugin.language_name().to_string(),
                version: lp.plugin.version().to_string(),
                default_output_dir: lp.plugin.default_output_dir().to_string(),
                file_extension: lp.plugin.file_extension().to_string(),
                source_path: lp.source_path.clone(),
            })
            .collect()
    }

    /// Check if any plugins are loaded.
    pub fn is_empty(&self) -> bool {
        self.plugins.is_empty()
    }

    /// Render a TypeDef using a plugin-provided mapper.
    pub fn render_with_plugin(
        &self,
        language_id: &str,
        type_def: &TypeDef,
        plugin_config: &PluginConfig,
        project_root: &Path,
    ) -> Result<Vec<GeneratedFile>> {
        let plugin = self.get(language_id).ok_or_else(|| {
            anyhow::anyhow!("no plugin registered for language '{}'", language_id)
        })?;

        let mapper = plugin.mapper(plugin_config);

        let output_dir_str = plugin_config
            .output_dir
            .as_deref()
            .unwrap_or(plugin.default_output_dir());
        let output_dir = project_root.join(output_dir_str);

        let filename = mapper.output_filename(type_def.name());
        let content = mapper.emit_type_def(type_def);

        Ok(vec![GeneratedFile {
            type_name: type_def.name().to_string(),
            language_label: plugin.language_id().to_string(),
            output_path: output_dir.join(filename),
            content,
            source_path: std::path::PathBuf::from("<plugin>"),
        }])
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a path looks like a shared library.
fn is_shared_library(path: &Path) -> bool {
    let ext = path.extension().and_then(|e| e.to_str()).unwrap_or("");
    matches!(ext, "so" | "dylib" | "dll")
}

/// Expand `~` to the user's home directory.
fn expand_tilde(path: &Path) -> PathBuf {
    let path_str = path.to_string_lossy();
    if path_str.starts_with("~/") {
        if let Ok(home) = std::env::var("HOME") {
            return PathBuf::from(format!("{}{}", home, &path_str[1..]));
        }
    }
    path.to_path_buf()
}

/// Build a plugin registry from typewriter configuration.
///
/// Loads plugins from the configured directory and explicit paths.
pub fn build_registry_from_config(
    config: &typewriter_core::config::TypewriterConfig,
) -> Result<PluginRegistry> {
    let mut registry = PluginRegistry::new();

    // Load from explicit paths
    let paths = config.plugin_paths();
    for path_str in paths {
        let path = PathBuf::from(path_str);
        registry.load_plugin(&path)?;
    }

    // Load from configured directory
    if let Some(dir) = config.plugin_dir() {
        let dir_path = expand_tilde(Path::new(dir));
        registry.load_from_dir(&dir_path)?;
    }

    // Load from default directory (~/.typewriter/plugins/) if it exists
    if let Ok(home) = std::env::var("HOME") {
        let default_dir = PathBuf::from(format!("{}/.typewriter/plugins", home));
        if default_dir.exists() {
            registry.load_from_dir(&default_dir)?;
        }
    }

    Ok(registry)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_registry() {
        let registry = PluginRegistry::new();
        assert!(registry.is_empty());
        assert!(registry.list().is_empty());
        assert!(registry.get("ruby").is_none());
    }

    #[test]
    fn test_is_shared_library() {
        assert!(is_shared_library(Path::new("libplugin.so")));
        assert!(is_shared_library(Path::new("libplugin.dylib")));
        assert!(is_shared_library(Path::new("plugin.dll")));
        assert!(!is_shared_library(Path::new("plugin.rs")));
        assert!(!is_shared_library(Path::new("plugin.toml")));
    }

    #[test]
    fn test_expand_tilde() {
        let path = Path::new("/absolute/path");
        assert_eq!(expand_tilde(path), PathBuf::from("/absolute/path"));
    }

    #[test]
    fn test_load_from_nonexistent_dir() {
        let mut registry = PluginRegistry::new();
        let result = registry.load_from_dir(Path::new("/nonexistent/plugins"));
        assert!(result.is_ok()); // should silently succeed
    }
}
