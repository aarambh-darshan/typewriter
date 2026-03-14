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
        assert_eq!(config.ts_output_dir(), "./generated/typescript");
        assert_eq!(config.py_output_dir(), "./generated/python");
        assert_eq!(config.go_output_dir(), "./generated/go");
        assert_eq!(config.go_package_name(), "types");
        assert!(!config.ts_readonly());
    }

    #[test]
    fn test_parse_full_config() {
        let toml_str = r#"
[typescript]
output_dir = "../frontend/src/types"
file_style = "kebab-case"
readonly = true

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
    }

    #[test]
    fn test_parse_empty_config() {
        let toml_str = "";
        let config: TypewriterConfig = toml::from_str(toml_str).unwrap();
        assert!(config.typescript.is_none());
        assert!(config.python.is_none());
    }

    #[test]
    fn test_load_nonexistent_file() {
        let config = TypewriterConfig::load(Path::new("/nonexistent/path")).unwrap();
        assert!(config.typescript.is_none());
    }
}
