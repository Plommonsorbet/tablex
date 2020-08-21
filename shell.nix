let
  moz_overlay = import (builtins.fetchTarball
    "https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz");
  nixpkgs = import <nixpkgs> { overlays = [ moz_overlay ]; };

  rust_nightly = (nixpkgs.latest.rustChannels.nightly.rust.override {
    extensions = [ "rust-src" "rust-analysis" ]

    ;
  });
in with nixpkgs;

stdenv.mkDerivation {
  name = "rust-nightly-dev";
  buildInputs = [
    rust_nightly
    rustup
    # to use the latest nightly:
    #nixpkgs.latest.rustChannels.nightly.rust 

    # to use a specific nighly:
    #(nixpkgs.rustChannelOf { channel = "nightly"; }).rust
    # to use the project's rust-toolchain file:
    #(nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust
    #(nixpkgs.rustChannelOf { rustToolchain = ./rust-toolchain; }).rust
  ];

}
#let
#  moz_overlay = import (builtins.fetchTarball https://github.com/mozilla/nixpkgs-mozilla/archive/master.tar.gz);
#  nixpkgs = import <nixpkgs> {
#    overlays = [ moz_overlay ];
#  };
#  ruststable = (nixpkgs.latest.rustChannels.stable.rust.override {
#    extensions = [ "rust-src" "rust-analysis" ];}
#  );
#in
#  with nixpkgs;
#  stdenv.mkDerivation {
#    name = "rust";
#    buildInputs = [ rustup ruststable ];
#  }
