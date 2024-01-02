{
  lib,
  glib,
  cargo,
  rustc,
  pkg-config,
  gtk4,
  pango,
  graphene,
  gtk4-layer-shell,
  openssl,
  pkgs ? import <nixpkgs> {},
}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    nativeBuildInputs = [
      cargo
      rustc
      pkg-config
    ];

    BuildInputs = [
      pango
      graphene
      glib
      gtk4
      gtk4-layer-shell
      openssl
    ];

    meta = with lib; {
      description = "A GTK4 status bar. in rust.";
      homepage = "https://github.com/elijahimmer/bar-rs";
      license = licenses.unlicense;
      maintainers = [];
      mainProgram = "bar-rs";
    };
  }
