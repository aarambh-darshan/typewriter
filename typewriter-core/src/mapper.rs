//! The `TypeMapper` trait — the core contract every language emitter must implement.
//!
//! Adding a new language to typewriter means implementing this one trait.

use crate::ir::*;

/// The core trait that every language emitter must implement.
///
/// This trait defines the full contract for converting typewriter's IR into
/// target language source code.
pub trait TypeMapper {
    /// Map a Rust primitive type to the target language's equivalent.
    ///
    /// # Examples
    /// - `PrimitiveType::String` → `"string"` (TS) or `"str"` (Python)
    /// - `PrimitiveType::U32` → `"number"` (TS) or `"int"` (Python)
    fn map_primitive(&self, ty: &PrimitiveType) -> String;

    /// Map an `Option<T>` type to the target language.
    ///
    /// # Examples
    /// - `Option<String>` → `"string | undefined"` (TS) or `"Optional[str]"` (Python)
    fn map_option(&self, inner: &TypeKind) -> String;

    /// Map a `Vec<T>` type to the target language.
    ///
    /// # Examples
    /// - `Vec<String>` → `"string[]"` (TS) or `"list[str]"` (Python)
    fn map_vec(&self, inner: &TypeKind) -> String;

    /// Map a `HashMap<K, V>` type to the target language.
    ///
    /// # Examples
    /// - `HashMap<String, u32>` → `"Record<string, number>"` (TS)
    fn map_hashmap(&self, key: &TypeKind, value: &TypeKind) -> String;

    /// Map a tuple type `(A, B, ...)` to the target language.
    fn map_tuple(&self, elements: &[TypeKind]) -> String;

    /// Map a named custom type (reference to another struct or enum).
    ///
    /// Usually the name is kept as-is in most languages.
    fn map_named(&self, name: &str) -> String;

    /// Render a complete struct as target language source code.
    fn emit_struct(&self, def: &StructDef) -> String;

    /// Render a complete enum as target language source code.
    fn emit_enum(&self, def: &EnumDef) -> String;

    /// File header content (auto-generated comment, import statements, pragmas).
    fn file_header(&self, type_name: &str) -> String;

    /// File footer content (if needed). Default: empty.
    fn file_footer(&self) -> String {
        String::new()
    }

    /// File extension for the output file (without the leading dot).
    fn file_extension(&self) -> &str;

    /// Naming convention for output files.
    ///
    /// # Examples
    /// - `"UserProfile"` → `"user-profile"` (kebab-case for TS)
    /// - `"UserProfile"` → `"user_profile"` (snake_case for Python)
    fn file_naming(&self, type_name: &str) -> String;

    /// Map any `TypeKind` to the target language string.
    ///
    /// This has a default implementation that dispatches to the specific mapping
    /// methods based on the variant. Override only if you need custom behavior.
    fn map_type(&self, ty: &TypeKind) -> String {
        match ty {
            TypeKind::Primitive(p) => self.map_primitive(p),
            TypeKind::Option(inner) => self.map_option(inner),
            TypeKind::Vec(inner) => self.map_vec(inner),
            TypeKind::HashMap(k, v) => self.map_hashmap(k, v),
            TypeKind::Tuple(elements) => self.map_tuple(elements),
            TypeKind::Named(name) => self.map_named(name),
            TypeKind::Generic(name, _params) => self.map_named(name),
            TypeKind::Unit => "void".to_string(),
        }
    }

    /// Emit a complete `TypeDef` (struct or enum) as target language source code.
    ///
    /// Includes the file header and footer wrapping the type definition.
    fn emit_type_def(&self, def: &TypeDef) -> String {
        let type_name = def.name();
        let mut output = self.file_header(type_name);

        match def {
            TypeDef::Struct(s) => output.push_str(&self.emit_struct(s)),
            TypeDef::Enum(e) => output.push_str(&self.emit_enum(e)),
        }

        let footer = self.file_footer();
        if !footer.is_empty() {
            output.push_str(&footer);
        }

        output
    }

    /// Get the full output filename for a type.
    fn output_filename(&self, type_name: &str) -> String {
        format!("{}.{}", self.file_naming(type_name), self.file_extension())
    }
}
