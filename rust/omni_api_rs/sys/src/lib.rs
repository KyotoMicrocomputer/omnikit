//! Rust bindings for the Omnisight API
#![expect(unsafe_code, reason = "FFI binding")]

include!(env!("GEN_PATH"));
pub use lib::*;
