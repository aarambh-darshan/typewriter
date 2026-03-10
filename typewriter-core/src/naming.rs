//! File naming utilities for converting type names to different case styles.
//!
//! Supports `kebab-case`, `snake_case`, and `PascalCase`.
//!
//! These functions are used by language emitters to determine output filenames
//! based on the `file_style` setting in `typewriter.toml`.

/// Supported file naming styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStyle {
    /// `UserProfile` → `user-profile` (default for TypeScript)
    KebabCase,
    /// `UserProfile` → `user_profile` (default for Python)
    SnakeCase,
    /// `UserProfile` → `UserProfile` (unchanged)
    PascalCase,
}

impl FileStyle {
    /// Parse a file style string from config.
    ///
    /// Returns `None` for unrecognized values.
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "kebab-case" => Some(FileStyle::KebabCase),
            "snake_case" => Some(FileStyle::SnakeCase),
            "PascalCase" | "pascalCase" | "pascal" => Some(FileStyle::PascalCase),
            _ => None,
        }
    }
}

/// Convert a PascalCase name to the specified file style.
///
/// # Examples
/// ```
/// use typewriter_core::naming::{FileStyle, to_file_style};
///
/// assert_eq!(to_file_style("UserProfile", FileStyle::KebabCase), "user-profile");
/// assert_eq!(to_file_style("UserProfile", FileStyle::SnakeCase), "user_profile");
/// assert_eq!(to_file_style("UserProfile", FileStyle::PascalCase), "UserProfile");
/// ```
pub fn to_file_style(name: &str, style: FileStyle) -> String {
    match style {
        FileStyle::KebabCase => to_kebab_case(name),
        FileStyle::SnakeCase => to_snake_case(name),
        FileStyle::PascalCase => name.to_string(),
    }
}

/// Convert PascalCase to kebab-case.
///
/// # Examples
/// - `"UserProfile"` → `"user-profile"`
/// - `"HTTPResponse"` → `"http-response"`
pub fn to_kebab_case(name: &str) -> String {
    convert_case(name, '-')
}

/// Convert PascalCase to snake_case.
///
/// # Examples
/// - `"UserProfile"` → `"user_profile"`
/// - `"HTTPResponse"` → `"http_response"`
pub fn to_snake_case(name: &str) -> String {
    convert_case(name, '_')
}

/// Internal helper — splits PascalCase using the given separator character.
fn convert_case(name: &str, separator: char) -> String {
    let mut result = String::new();
    for (i, c) in name.chars().enumerate() {
        if c.is_uppercase() {
            if i > 0 {
                let prev_lower = name.chars().nth(i - 1).map_or(false, |p| p.is_lowercase());
                let next_lower = name.chars().nth(i + 1).map_or(false, |n| n.is_lowercase());
                if prev_lower || next_lower {
                    result.push(separator);
                }
            }
            result.push(c.to_lowercase().next().unwrap());
        } else {
            result.push(c);
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kebab_case() {
        assert_eq!(to_kebab_case("UserProfile"), "user-profile");
        assert_eq!(to_kebab_case("User"), "user");
        assert_eq!(to_kebab_case("HTTPResponse"), "http-response");
        assert_eq!(to_kebab_case("MyAPIKey"), "my-api-key");
    }

    #[test]
    fn test_snake_case() {
        assert_eq!(to_snake_case("UserProfile"), "user_profile");
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("HTTPResponse"), "http_response");
        assert_eq!(to_snake_case("MyAPIKey"), "my_api_key");
    }

    #[test]
    fn test_pascal_case() {
        assert_eq!(
            to_file_style("UserProfile", FileStyle::PascalCase),
            "UserProfile"
        );
        assert_eq!(
            to_file_style("HTTPResponse", FileStyle::PascalCase),
            "HTTPResponse"
        );
    }

    #[test]
    fn test_to_file_style() {
        assert_eq!(
            to_file_style("UserProfile", FileStyle::KebabCase),
            "user-profile"
        );
        assert_eq!(
            to_file_style("UserProfile", FileStyle::SnakeCase),
            "user_profile"
        );
        assert_eq!(
            to_file_style("UserProfile", FileStyle::PascalCase),
            "UserProfile"
        );
    }

    #[test]
    fn test_file_style_from_str() {
        assert_eq!(
            FileStyle::from_str("kebab-case"),
            Some(FileStyle::KebabCase)
        );
        assert_eq!(
            FileStyle::from_str("snake_case"),
            Some(FileStyle::SnakeCase)
        );
        assert_eq!(
            FileStyle::from_str("PascalCase"),
            Some(FileStyle::PascalCase)
        );
        assert_eq!(FileStyle::from_str("unknown"), None);
    }
}
