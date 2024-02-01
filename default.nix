{pkgs ? import "<nixpkgs>" {}, ...}: let
  manifest = (pkgs.lib.importTOML ./Cargo.toml).package;
in
  pkgs.rustPlatform.buildRustPackage {
    pname = manifest.name;
    version = manifest.version;
    cargoLock.lockFile = ./Cargo.lock;
    src = pkgs.lib.cleanSource ./.;

    nativeBuildInputs = with pkgs; [
      cargo
      rustc
      pkg-config
    ];

    buildInputs = with pkgs; [
      glib
      gtk4
      gtk4-layer-shell
    ];

    meta = with pkgs.lib; {
      description = "A GTK4 status bar. in rust.";
      homepage = "https://github.com/elijahimmer/bar-rs";
      license = licenses.mit;
      maintainers = [];
      mainProgram = "bar-rs";
    };
  }
