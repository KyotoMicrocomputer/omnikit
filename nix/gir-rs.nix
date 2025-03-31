{
  pkgs,
  naersk,
}:

let
  naersk' = pkgs.callPackage naersk { };
in
naersk'.buildPackage {
  src = ../vendor/gtk-rs-gir;

  # The vendored crate's build script reads .git/HEAD, which is absent from
  # the source copied into the Nix build sandbox.
  overrideMain = _: {
    postPatch = ''
      substituteInPlace build.rs \
        --replace-fail '    let repo_path = git::git_dir(".").unwrap();' \
        '    let Some(repo_path) = git::git_dir(".") else {
          std::fs::write("src/gir_version.rs", "pub const VERSION: &str = \"???\";").unwrap();
          return;
      };'
    '';
  };
}
