/// The macro invoked in each `gir`-generated module
/// `[tagref redacted]`
macro_rules! patch_auto {
    // No file-specific rules are defined at the moment

    // Default rule
    (
        // Module file name (e.g., `"value_map.rs"`)
        $file_name:literal;
        // Original source tokens
        $($tt:tt)*
    ) => {
        #[allow(unused_imports)]
        pub use crate::translate::IntoGPtrArray;

        $($tt)*
    };
}
