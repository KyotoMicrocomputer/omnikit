#!/usr/bin/env bash

set -euo pipefail

repo_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)
# [tag:etc_gi_docgen_out_dir]
out_dir="$repo_dir/build/doc"

usage() {
    printf 'Usage: %s [-o OUTPUT_DIR]\n' "$0" >&2
}

while getopts ':o:h' opt; do
    case "$opt" in
        o) out_dir=$OPTARG ;;
        h) usage; exit 0 ;;
        *) usage; exit 2 ;;
    esac
done
shift $((OPTIND - 1))
if (( $# != 0 )); then
    usage
    exit 2
fi

# =================================================================
# Locate `GLib-2.0.gir`
# =================================================================

# [ref:doc_prerequisite_gir]

# Note: <https://github.com/gtk-rs/gir-files> is ineligible because
# it is missing the `filename` attribute, which cause gi-docgen to
# crash.

gir_dir=
# Nix
# <https://github.com/NixOS/nixpkgs/blob/ac588ef1c914865dad517b337ea0eb9694a4021c/pkgs/by-name/gl/glib/package.nix#L206>
# <https://github.com/NixOS/nixpkgs/blob/ac588ef1c914865dad517b337ea0eb9694a4021c/pkgs/development/libraries/gobject-introspection/setup-hook.sh#L36>
#
# MSYS2 UCRT
# <https://packages.msys2.org/packages/mingw-w64-ucrt-x86_64-glib2>
if includedir=$(pkg-config --variable=includedir glib-2.0 2>/dev/null); then
    candidate="$includedir/../share/gir-1.0"
    if [[ -f "$candidate/GLib-2.0.gir" ]]; then
        gir_dir=$candidate
    fi
fi

# <https://packages.debian.org/trixie/libgirepository1.0-dev>
if [[ -z "$gir_dir" ]] && candidate=$(pkg-config --variable=girdir gobject-introspection-1.0 2>/dev/null); then
    if [[ -f "$candidate/GLib-2.0.gir" ]]; then
        gir_dir=$candidate
    fi
fi

if [[ -z "$gir_dir" ]]; then
    printf 'Could not find GLib-2.0.gir using pkg-config.\n' >&2
    exit 1
fi

# =================================================================
# Generate
# =================================================================

set -x
# [ref:doc_prerequisite_gi_docgen]
"${GI_DOCGEN:-gi-docgen}" generate "$repo_dir/meta/Omni-0.0.gir" \
    --add-include-path "$gir_dir" \
    --output-dir "$out_dir" \
    --quiet
