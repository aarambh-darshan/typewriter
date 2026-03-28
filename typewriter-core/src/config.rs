//! Configuration parsing for `typewriter.toml`.
//!
//! All fields are optional — sensible defaults are used when not specified.

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::Path;

/// Top-level configuration from `typewriter.toml`.
#[derive(Debug, Clone, Deserialize, Default)]
pub struct TypewriterConfig {
    /// TypeScript emitter configuration
    pub typescript: Option<TypeScriptConfig>,
    /// Python emitter configuration
    pub python: Option<PythonConfig>,
    /// Go emitter configuration
    pub go: Option<GoConfig>,
    /// Swift emitter configuration
    pub swift: Option<SwiftConfig>,
    /// Kotlin emitter configuration
    pub kotlin: Option<KotlinConfig>,
    /// GraphQL emitter configuration
    pub graphql: Option<GraphQLConfig>,
}

/// TypeScript-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct TypeScriptConfig {
    /// Output directory for generated `.ts` files
    pub output_dir: Option<String>,
    /// File naming style: `kebab-case` (default), `snake_case`, `PascalCase`
    pub file_style: Option<String>,
    /// If true, all fields become `readonly`
    pub readonly: Option<bool>,
    /// If false, skip generating sibling `.schema.ts` Zod files for TypeScript
    pub zod: Option<bool>,
}

/// Python-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct PythonConfig {
    /// Output directory for generated `.py` files
    pub output_dir: Option<String>,
    /// File naming style: `snake_case` (default), `kebab-case`, `PascalCase`
    pub file_style: Option<String>,
    /// Use Pydantic v2 BaseModel (default: true)
    pub pydantic_v2: Option<bool>,
    /// Use `@dataclass` instead of BaseModel
    pub use_dataclass: Option<bool>,
}

/// Go-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct GoConfig {
    /// Output directory for generated `.go` files
    pub output_dir: Option<String>,
    /// File naming style: `snake_case` (default), `kebab-case`, `PascalCase`
    pub file_style: Option<String>,
    /// Package name for generated files (default: "types")
    pub package_name: Option<String>,
}

/// Swift-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct SwiftConfig {
    /// Output directory for generated `.swift` files
    pub output_dir: Option<String>,
    /// File naming style: `PascalCase` (default), `snake_case`, `kebab-case`
    pub file_style: Option<String>,
}

/// Kotlin-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct KotlinConfig {
    /// Output directory for generated `.kt` files
    pub output_dir: Option<String>,
    /// File naming style: `PascalCase` (default), `snake_case`, `kebab-case`
    pub file_style: Option<String>,
    /// Package name for generated files (default: "types")
    pub package_name: Option<String>,
}

/// GraphQL-specific configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct GraphQLConfig {
    /// Output directory for generated `.graphql` files
    pub output_dir: Option<String>,
    /// File naming style: `snake_case` (default), `kebab-case`, `PascalCase`
    pub file_style: Option<String>,
}

impl TypewriterConfig {
    /// Load configuration from a `typewriter.toml` file.
    ///
    /// Returns `Ok(default)` if the file doesn't exist (all fields optional).
    /// Returns `Err` if the file exists but is malformed.
    pub fn load(project_root: &Path) -> Result<Self> {
        let config_path = project_root.join("typewriter.toml");

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = std::fs::read_to_string(&config_path)
            .with_context(|| format!("Failed to read {}", config_path.display()))?;

        let config: TypewriterConfig = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", config_path.display()))?;

        Ok(config)
    }

    /// Get the TypeScript output directory, with a default fallback.
    pub fn ts_output_dir(&self) -> &str {
        self.typescript
            .as_ref()
            .and_then(|ts| ts.output_dir.as_deref())
            .unwrap_or("./generated/typescript")
    }

    /// Get the Python output directory, with a default fallback.
    pub fn py_output_dir(&self) -> &str {
        self.python
            .as_ref()
            .and_then(|py| py.output_dir.as_deref())
            .unwrap_or("./generated/python")
    }

    /// Check if TypeScript readonly mode is enabled.
    pub fn ts_readonly(&self) -> bool {
        self.typescript
            .as_ref()
            .and_then(|ts| ts.readonly)
            .unwrap_or(false)
    }

    /// Check if TypeScript Zod schema generation is enabled.
    pub fn ts_zod_enabled(&self) -> bool {
        self.typescript
            .as_ref()
            .and_then(|ts| ts.zod)
            .unwrap_or(true)
    }

    /// Get the TypeScript file naming style.
    ///
    /// Returns the parsed `FileStyle` from config, or `None` to use the emitter's default.
    pub fn ts_file_style(&self) -> Option<crate::naming::FileStyle> {
        self.typescript
            .as_ref()
            .and_then(|ts| ts.file_style.as_deref())
            .and_then(crate::naming::FileStyle::from_str)
    }

    /// Get the Python file naming style.
    ///
    /// Returns the parsed `FileStyle` from config, or `None` to use the emitter's default.
    pub fn py_file_style(&self) -> Option<crate::naming::FileStyle> {
        self.python
            .as_ref()
            .and_then(|py| py.file_style.as_deref())
            .and_then(crate::naming::FileStyle::from_str)
    }

    /// Get the Go output directory, with a default fallback.
    pub fn go_output_dir(&self) -> &str {
        self.go
            .as_ref()
            .and_then(|go| go.output_dir.as_deref())
            .unwrap_or("./generated/go")
    }

    /// Get the Go file naming style.
    ///
    /// Returns the parsed `FileStyle` from config, or `None` to use the emitter's default.
    pub fn go_file_style(&self) -> Option<crate::naming::FileStyle> {
        self.go
            .as_ref()
            .and_then(|go| go.file_style.as_deref())
            .and_then(crate::naming::FileStyle::from_str)
    }

    /// Get the Go package name.
    pub fn go_package_name(&self) -> &str {
        self.go
            .as_ref()
            .and_then(|go| go.package_name.as_deref())
            .unwrap_or("types")
    }

    /// Get the Swift output directory, with a default fallback.
    pub fn swift_output_dir(&self) -> &str {
        self.swift
            .as_ref()
            .and_then(|s| s.output_dir.as_deref())
            .unwrap_or("./generated/swift")
    }

    /// Get the Swift file naming style.
    pub fn swift_file_style(&self) -> Option<crate::naming::FileStyle> {
        self.swift
            .as_ref()
            .and_then(|s| s.file_style.as_deref())
            .and_then(crate::naming::FileStyle::from_str)
    }

    /// Get the Kotlin output directory, with a default fallback.
    pub fn kotlin_output_dir(&self) -> &str {
        self.kotlin
            .as_ref()
            .and_then(|k| k.output_dir.as_deref())
            .unwrap_or("./generated/kotlin")
    }

    /// Get the Kotlin file naming style.
    pub fn kotlin_file_style(&self) -> Option<crate::naming::FileStyle> {
        self.kotlin
            .as_ref()
            .and_then(|k| k.file_style.as_deref())
            .and_then(crate::naming::FileStyle::from_str)
    }

    /// Get the Kotlin package name.
    pub fn kotlin_package_name(&self) -> &str {
        self.kotlin
            .as_ref()
            .and_then(|k| k.package_name.as_deref())
            .unwrap_or("types")
    }

    /// Get the GraphQL output directory, with a default fallback.
    pub fn graphql_output_dir(&self) -> &str {
        self.graphql
            .as_ref()
            .and_then(|g| g.output_dir.as_deref())
            .unwrap_or("./generated/graphql")
    }

    /// Get the GraphQL file naming style.
    pub fn graphql_file_style(&self) -> Option<crate::naming::FileStyle> {
        self.graphql
            .as_ref()
            .and_then(|g| g.file_style.as_deref())
            .and_then(crate::naming::FileStyle::from_str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = TypewriterConfig::default();
        assert!(config.typescript.is_none());
        assert!(config.python.is_none());
        assert!(config.go.is_none());
        assert!(config.swift.is_none());
        assert!(config.kotlin.is_none());
        assert!(config.graphql.is_none());
        assert_eq!(config.ts_output_dir(), "./generated/typescript");
        assert_eq!(config.py_output_dir(), "./generated/python");
        assert_eq!(config.go_output_dir(), "./generated/go");
        assert_eq!(config.go_package_name(), "types");
        assert_eq!(config.swift_output_dir(), "./generated/swift");
        assert_eq!(config.kotlin_output_dir(), "./generated/kotlin");
        assert_eq!(config.kotlin_package_name(), "types");
        assert_eq!(config.graphql_output_dir(), "./generated/graphql");
        assert!(!config.ts_readonly());
        assert!(config.ts_zod_enabled());
    }

    #[test]
    fn test_parse_full_config() {
        let toml_str = r#"
[typescript]
output_dir = "../frontend/src/types"
file_style = "kebab-case"
readonly = true
zod = false

[python]
output_dir = "../api/schemas"
pydantic_v2 = true

[go]
output_dir = "../backend/types"
package_name = "api_types"
"#;
        let config: TypewriterConfig = toml::from_str(toml_str).unwrap();

        assert_eq!(
            config.typescript.as_ref().unwrap().output_dir.as_deref(),
            Some("../frontend/src/types")
        );
        assert_eq!(
            config.typescript.as_ref().unwrap().file_style.as_deref(),
            Some("kebab-case")
        );
        assert_eq!(config.typescript.as_ref().unwrap().readonly, Some(true));
        assert_eq!(config.typescript.as_ref().unwrap().zod, Some(false));
        assert!(!config.ts_zod_enabled());
        assert_eq!(
            config.python.as_ref().unwrap().output_dir.as_deref(),
            Some("../api/schemas")
        );
        assert_eq!(config.python.as_ref().unwrap().pydantic_v2, Some(true));
        assert_eq!(
            config.go.as_ref().unwrap().output_dir.as_deref(),
            Some("../backend/types")
        );
        assert_eq!(
            config.go.as_ref().unwrap().package_name.as_deref(),
            Some("api_types")
        );
    }

    #[test]
    fn test_parse_partial_config() {
        let toml_str = r#"
[typescript]
output_dir = "../frontend/types"
"#;
        let config: TypewriterConfig = toml::from_str(toml_str).unwrap();

        assert_eq!(config.ts_output_dir(), "../frontend/types");
        assert_eq!(config.py_output_dir(), "./generated/python");
        assert!(!config.ts_readonly());
        assert!(config.ts_zod_enabled());
    }

    #[test]
    fn test_parse_empty_config() {
        let toml_str = "";
        let config: TypewriterConfig = toml::from_str(toml_str).unwrap();
        assert!(config.typescript.is_none());
        assert!(config.python.is_none());
        assert!(config.ts_zod_enabled());
    }

    #[test]
    fn test_load_nonexistent_file() {
        let config = TypewriterConfig::load(Path::new("/nonexistent/path")).unwrap();
        assert!(config.typescript.is_none());
    }
}
