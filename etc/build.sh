#!/usr/bin/env bash

# [ref:codegen_build_sh]
# Script to invoke all pregeneration scripts

set -euo pipefail
cd "$(dirname "$0")/.."

usage() {
    printf 'Usage: %s {generate|clean|dist}\n' "$0" >&2
}

generate() {
    rust/build.sh generate
}

clean() {
    rust/build.sh clean
}

dist() {
    if [[ "${OMNIKIT_DIST:-}" == "" ]]; then
        printf 'dist requires OMNIKIT_DIST to be set\n' >&2
        exit 1
    fi

    out="$OMNIKIT_DIST"
    set -x

    mkdir -p "$out"
    cp -R meta "$out/"

    rust/build.sh dist

    # Redact Tagref markers from all distributed files so that they are not
    # flagged when the files are vendored in a downstream project.
    find "$out" -type f -exec sed -i -E 's/\[(tag|ref|dir|file):[^]]*\]/[tagref redacted]/g' {} +
    tagref -p "$out"
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
