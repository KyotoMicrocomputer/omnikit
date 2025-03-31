# OmniKit

A plugin SDK (software development kit) for KMC Prism Profiler.

> [!WARNING]
> This is a preview version.
> - Breaking changes to features and ABI may occur.
> - Git branches may be removed or have their commit histories cleared.

## Branches

The **`main`** branch contains the source code and raw API metadata.
<!-- [tag:git_branch_main] -->

CI automatically pushes to the **`pregenerated`** branch whenever `main` is updated.
It contains binding libraries with pregenerated code, ready for use in plugin development.
<!-- [tag:git_branch_pregenerated] -->

## Directory structure

This section describes the role and purpose of each file in this repository.

### Plugin API metadata

The following files contain raw API metadata generated from the canonical implementation.

- **`meta/Omni-*.gir`** contains the GIR meadata for the plugin API, from which binding libraries can be generated automatically for various programming languages.
- **`meta/omnikit.h`** is the C header file for the plugin API.

### Binding libraries

The following directories contain binding libraries that can be used to implement plugins in different programming langauges.

- **`rust/`** contains the [Rust][5] binding library.
  This is officially supported.

### Build infrastructure

The following files are used to build and develop OmniKit itself.

- **`flake.nix`** defines a [Nix flake][1] providing a build environment for OmniKit.
  It can be used with [the Nix package manager][2].

- **`nix/`** contains helper code for `flake.nix`.

- **`etc/`** contains helper tools used during development.

- **`vendor/`** contains vendored external source files.

- **`.github/workflows/`** contains configuration files for [GitHub Actions][3] CI, which automatically runs actions on git pushes, pull requests, and other triggers.

- **`docs/`** contains additional documentation for OmniKit developers.

  **Documentation for plugin developers is provided separately.**
- **`.editorconfig`** is an [EditorConfig][4] file. Some text editors read this file to determine the default indentation settings.

## Developing

See [this guide](docs/developing.md) for how to develop OmniKit.

## Policies

- The directory structure may change without prior notice.

[1]: https://nix.dev/manual/nix/2.29/command-ref/new-cli/nix3-flake.html
[2]: https://nix.dev/
[3]: https://github.com/features/actions
[4]: https://editorconfig.org/
[5]: https://rust-lang.org/
