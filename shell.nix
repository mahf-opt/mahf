{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell {
  buildInputs = with pkgs; [
    rustc
    cargo
    rustfmt
    rust-analyzer
    clippy
    clang
  ];

  RUST_BACKTRACE = 1;
  RUST_SRC_PATH = "${pkgs.rustPlatform.rustLibSrc}";
}