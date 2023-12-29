#pkg-config pango graphene gtk4
{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  # nativeBuildInputs is usually what you want -- tools you need to run
  nativeBuildInputs = with pkgs.buildPackages; [
    cargo
    clippy
    rust-analyzer
    rustfmt

    pkg-config
    pango
    graphene
    gtk4
    gtk4-layer-shell
  ];
}
