//! Rust FFI library for the Omnisight API
#![expect(unsafe_code, reason = "FFI binding")]

pub use ffi;

#[macro_use]
mod auto_prelude;

#[allow(clippy::all)]
#[allow(unused_imports)]
#[allow(unused_variables)]
#[allow(rustdoc::redundant_explicit_links)]
mod auto {
    include!(env!("GEN_PATH"));
    pub use auto::*;
}
pub use auto::*;
pub mod functions {
    pub use crate::auto::functions::*;
}

pub mod builders {
    pub use crate::auto::builders::*;
}

pub mod color;
pub mod link_array;
pub mod link_policy;
pub mod plugin_registry;
mod port_link_set;
pub mod prelude;
pub mod subclass;
pub mod time;
pub mod translate;

// The enum is defined in generated code
#[expect(clippy::derivable_impls)]
impl Default for SortOrder {
    fn default() -> Self {
        Self::None
    }
}
