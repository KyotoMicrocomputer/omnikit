{ pkgs }:
let
  channel = (builtins.fromTOML (builtins.readFile ../rust/rust-toolchain.toml)).toolchain.channel;
in
{
  toolchain =
    if pkgs.rustc.version != channel then
      throw "nixpkgs rustc ${pkgs.rustc.version} does not match rust/rust-toolchain.toml (${channel})"
    else
      pkgs.symlinkJoin {
        name = "rust-toolchain";
        paths = with pkgs; [
          cargo
          rustc
          clippy
          rustfmt
        ];
      };
}
