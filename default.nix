{
  lib,
  cargo,
  rustc,
  pkg-config,
  glib,
  gtk4,
  gtk4-layer-shell,
  pkgs,
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

    buildInputs = [
      glib
      gtk4
      gtk4-layer-shell
    ];

    meta = with lib; {
      description = "A GTK4 status bar. in rust.";
      homepage = "https://github.com/elijahimmer/bar-rs";
      license = licenses.mit;
      maintainers = [];
      mainProgram = "bar-rs";
    };
  }
