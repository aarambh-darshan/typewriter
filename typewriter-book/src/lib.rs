//! # typewriter-book
//!
//! This is a placeholder crate for the typewriter mdBook documentation.
//!
//! To build the guide, use:
//!
//! ```bash
//! cargo install mdbook
//! mdbook build typewriter-book
//! ```
//!
//! Or serve locally:
//!
//! ```bash
//! mdbook serve typewriter-book
//! ```

#[cfg(test)]
mod tests {
    #[test]
    fn crate_docs_mention_mdbook() {
        let docs = env!("CARGO_PKG_DESCRIPTION");
        assert!(docs.contains("mdBook"));
    }
}
