#!/usr/bin/env bash

# Script to enforce various sueprficial invariants in the source tree

set -euo pipefail
cd "$(dirname "$0")/.."

# enable `**`
shopt -s globstar

# Detect broken references in `[ref:tagref]`
tagref

# =====================================================================
# Check formatting
# =====================================================================
pushd rust; cargo fmt --check; popd

# [ref:doc_prerequisite_nixfmt]
if command -v nixfmt; then
    nixfmt **/*.nix
else
    echo "warning: nixfmt not found, skipping"
fi

# [ref:doc_prerequisite_actionlint]
if command -v actionlint; then
    # allow `if: false`
    actionlint -ignore 'constant expression "false" in condition'
else
    echo "warning: actioncheck not found, skipping"
fi

echo "tidy check result: ok."
