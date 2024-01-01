{lib, pkgs ? import <nixpkgs> {}}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage rec {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;


    nativeBuildInputs = with pkgs; [
      cargo
      cairo
      rustc
      pkg-config
      pango
      graphene
      glib
      glib.dev
      gtk4
      gtk4-layer-shell
    ];

    meta = with lib; {
      description = "A GTK4 status bar. in rust.";
      homepage = "https://github.com/elijahimmer/bar-rs";
      license = licenses.unlicense;
      maintainers = [];
      mainProgram = "bar-rs";
    };
  }
