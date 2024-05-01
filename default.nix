let
  rust_overlay = import (builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "1.77.2";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "rust-analyzer"
    ];
  };
in
pkgs.mkShell {
  buildInputs = [
    rust
  ] ++ (with pkgs; [
    pkg-config
    protobuf
  ]);
  RUST_BACKTRACE = 1;
}