let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };
in with nixpkgs;
stdenv.mkDerivation {
  name = "moz_overlay_shell";
  buildInputs = [
    # to use the latest stable:
    nixpkgs.latest.rustChannels.stable.rust
    # to use the project's rust-toolchain file:
    (nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain.toml; }).rust
  ];
}

