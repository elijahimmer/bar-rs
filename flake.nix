{
  description = "A GTK4 Wayland Status Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    # Very nice to use
    flake-utils.url = "github:numtide/flake-utils";

    # Great rust build system
    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }: let
    supportedSystems = with flake-utils.lib.system; [
      x86_64-linux
      aarch64-linux
      # no mac, wayland isn't on mac (as far as I know...)
      # also, bsd users can fix this themselves. There are too many options...
    ];
  in
    (flake-utils.lib.eachSystem supportedSystems (system: let
      pkgs = (import nixpkgs) {
        inherit system;
      };

      naersk' = pkgs.callPackage naersk {};

      buildInputs = with pkgs; [
        pkg-config
        glib
        gtk4
        gtk4-layer-shell
      ];
    in {
      packages.default = naersk'.buildPackage {
        inherit buildInputs;
        src = ./.;
        meta = with pkgs.lib; {
          description = "A GTK4 status bar. in rust.";
          homepage = "https://github.com/elijahimmer/bar-rs";
          license = licenses.mit;
          mainProgram = "bar-rs";
        };
      };

      devShells.default = pkgs.mkShell {
        buildInputs =
          buildInputs
          ++ (with pkgs; [
            cargo
            rustc
            clippy
          ]);
      };

      formatter = pkgs.alejandra;
    }))
    // {
      nixosModules = rec {
        bar-rs = default;
        default = {
          config,
          lib,
          pkgs,
          system,
          ...
        }: let
          cfg = config.services.bar-rs;
        in
          with lib; {
            options = {
              services.bar-rs = {
                enable = mkOption {
                  type = types.bool;
                  default = false;
                  example = true;
                  description = mdDoc ''
                    Enables bar-rs to run when your window manager starts
                  '';
                };
                targets = mkOption {
                  type = types.listOf types.str;
                  default = ["graphical-session.target"];
                  example = ["hyprland-session.target"];
                };
              };
            };

            config = mkIf cfg.enable {
              systemd = {
                user = {
                  services.bar-rs = {
                    wantedBy = cfg.targets;
                    script = getExe self.packages.${system}.default;
                    reloadIfChanged = true;
                    environment = {
                      pw-volume = pkgs.pw-volume;
                    };
                  };
                };
              };
            };
          };
      };
    };
}
