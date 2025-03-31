#!/usr/bin/env bash

set -euo pipefail
cd "$(dirname "$0")"

# [ref:codegen_build_sh]
# Implementation of the pre-generation script for the Rust binding library

usage() {
    printf 'Usage: %s {generate|clean|dist}\n' "$0" >&2
}

generate() {
    # Call `omni_api_builnd::generate_bindings` through
    # [ref:omni_api_generate_bindings] to generate binding code.
    # Place the generated code in `gen` directories [tag:rust_gen_dir]
    cargo run --bin omni_api_generate_bindings -- ffi $PWD/omni_api_rs/sys/gen
    cargo run --bin omni_api_generate_bindings -- api $PWD/omni_api_rs/gen
}

clean() {
    # Remove the directories created in [ref:rust_gen_dir]
    rm -rf omni_api_rs/sys/gen
    rm -rf omni_api_rs/gen
}

dist() {
    if [[ "${OMNIKIT_DIST:-}" == "" ]]; then
        printf 'dist requires OMNIKIT_DIST to be set\n' >&2
        exit 1
    fi

    out="$OMNIKIT_DIST/rust"
    set -x

    mkdir -p "$out"
    cp -R Cargo.{toml,lock} "$out/"
    cp -R rust-toolchain.toml "$out/"
    cp -R omni_api_rs "$out/"
    cp -R sample "$out/"

    # Remove references to `omni_api_build`, which is not included in output
    for file in "$out"/omni_api_rs/{,sys/}Cargo.toml; do
        # Remove the Cargo dependency
        sed -i 's/omni_api_build = .*//g' "$file"

        # Disable the `_internal_generate` Cargo feature
        # `default = ["_internal_generate"]` -> `default = []`
        sed -i 's/"_internal_generate"//g' "$file"
    done

    sed -i 's/"omni_api_build",//g' "$out/Cargo.toml"

    # Remove references to `omni_api_stub`, which is not included in output
    sed -i 's/"omni_api_stub",//g' "$out/Cargo.toml"

    # Remove workspace dependencies, too, to suppress
    # `cargo::unused_workspace_dependencies`
    sed -i 's/omni_api_build.*//g' "$out/Cargo.toml"
    sed -i 's/xot.*//g' "$out/Cargo.toml"
    sed -i 's/tempfile.*//g' "$out/Cargo.toml"
}

if [[ $# -ne 1 ]]; then
    usage
    exit 2
fi

case "$1" in
    generate)
        generate
        ;;
    clean)
        clean
        ;;
    dist)
        dist
        ;;
    *)
        usage
        exit 2
        ;;
esac
