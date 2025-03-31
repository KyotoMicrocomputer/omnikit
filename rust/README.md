# Rust bindings for OmniKit

A Rust binding library for OmniKit based on [the `glib` crate][2].

## Directory structure

- **`build.sh`** implements the pre-generation script, following the interface defined in `[ref:codegen_build_sh]`.

- **`Cargo.toml`** defines a Cargo [workspace][3].

- **`Cargo.lock`** records the last selected versions for Cargo dependencies.

- **`omni_api_rs/`** contains the binding library.

- **`omni_api_build/`** contains the code generation helper used by `omni_api_rs` and `build.sh`.

- **`omni_api_stub/`** provides a test-only shared library that stands in for the real `libomnikit.so`.

- **`rust-toolchain.toml`** [instructs][4] rustup to use a specific version of the Rust toolchain.

- **`.rustfmt.toml`** is a [rustfmt][1] (Rust code formatter) configuration file.

- **`sample/`** contains a sample plugin written in Rust.

## Policies

- GLib binding versioning policy: Although the native GLib library has been stable for decades, its Rust binding, the `glib` crate, is frequently updated with breaking changes.
  In OmniKit, we generally support only the latest version of the `glib` crate.

[1]: https://github.com/rust-lang/rustfmt
[2]: https://crates.io/crates/glib
[3]: https://doc.rust-lang.org/cargo/appendix/glossary.html#workspace
[4]: https://rust-lang.github.io/rustup/overrides.html
