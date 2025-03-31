{
  inputs = {
    self.submodules = true;
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-26.05";
    naersk = {
      url = "github:nix-community/naersk/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      utils,
      naersk,
    }:
    utils.lib.eachSystem utils.lib.allSystems (
      system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
        rust = import ./nix/rust.nix {
          inherit pkgs;
        };

        # Packages that are still required in pre-generated state
        # [ref:codegen_pregen_state]
        userBuildInputs = [
          # [ref:doc_prerequisite_glib]
          # [ref:doc_prerequisite_gir]
          pkgs.glib
          # [ref:doc_prerequisite_pkg_config]
          pkgs.pkg-config
          # [dir:rust]
          rust.toolchain
        ];

        # Additional packages required for full development, including building
        # bindings in on-demand generation state [ref:codegen_ondemand_state]
        developBuildInputs = [
          # [ref:doc_prerequisite_nixfmt]
          pkgs.nixfmt
          # [ref:doc_prerequisite_actionlint]
          pkgs.actionlint
          # [ref:tagref]
          pkgs.tagref
          # Needed by [ref:api_build_prerequisite_gir]
          (pkgs.callPackage ./nix/gir-rs.nix { inherit naersk; })
          # [ref:doc_prerequisite_gi_docgen]
          pkgs.gi-docgen
        ];

      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = userBuildInputs ++ developBuildInputs;

          LD_LIBRARY_PATH = pkgs.lib.makeLibraryPath [
            pkgs.glib
          ];
        };

        # [tag:flake_devshell_doc]
        # Includes only packages necessary for [file:etc/gi-docgen.sh]
        devShells.doc = pkgs.mkShell {
          buildInputs = [
            # [ref:doc_prerequisite_gir]
            pkgs.glib
            # [ref:doc_prerequisite_pkg_config]
            pkgs.pkg-config
            # [ref:doc_prerequisite_gi_docgen]
            pkgs.gi-docgen
          ];
        };
      }
    );
}
