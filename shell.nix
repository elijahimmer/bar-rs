{pkgs ? import <nixpkgs> {}}:
pkgs.mkShell {
  name = "nix";
  nativeBuildInputs = with pkgs; [
    cargo
    rustc
    pkg-config
    hyprland
  ];

  buildInputs = with pkgs; [
    glib
    gtk4
    gtk4-layer-shell
  ];
}
