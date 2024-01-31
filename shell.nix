{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "nix";
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    clippy
    pkg-config
    pw-volume
  ];

  buildInputs = with pkgs; [
    glib
    gtk4
    gtk4-layer-shell
  ];
}
