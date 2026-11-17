//! The public root keys of Peios Signed Objects, for a verifier written in
//! Rust to embed at build time.
//!
//! A verifier that depends on this crate pins it by commit in its
//! `Cargo.lock`, so a root is added to that verifier only when its lock is
//! updated.

#![no_std]

/// Each root: its file's name in `roots/`, and the file's content, which is
/// a public key in the form `pso root` writes.
pub const ROOTS: &[(&str, &str)] = &[
    ("peios-root-1.pub", include_str!("../roots/peios-root-1.pub")),
    ("peios-root-2.pub", include_str!("../roots/peios-root-2.pub")),
];

#[cfg(test)]
mod tests {
    extern crate std;

    use std::collections::BTreeSet;
    use std::string::String;

    /// A root added to `roots/` and not to `ROOTS` would be published and
    /// never embedded.
    #[test]
    fn every_file_in_roots_is_embedded() {
        let directory = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("roots");
        let on_disk: BTreeSet<String> = std::fs::read_dir(directory)
            .unwrap()
            .map(|entry| entry.unwrap().file_name().into_string().unwrap())
            .collect();
        let embedded: BTreeSet<String> =
            super::ROOTS.iter().map(|(name, _)| String::from(*name)).collect();
        assert_eq!(on_disk, embedded);
    }
}
