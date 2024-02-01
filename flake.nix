{
  description = "A GTK4 Wayland Status Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";

    naersk.url = "github:nmattia/naersk";
    naersk.inputs.nixpkgs.follows = "nixpkgs";
  };
  outputs = {
    self,
    flake-utils,
    naersk,
    nixpkgs,
  }:
    flake-utils.lib.eachDefaultSystem (system: let
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
      defaultPackage = naersk'.buildPackage {
        inherit buildInputs;
        src = ./.;
        meta = with pkgs.lib; {
          description = "A GTK4 status bar. in rust.";
          homepage = "https://github.com/elijahimmer/bar-rs";
          license = licenses.mit;
          mainProgram = "bar-rs";
        };
      };

      devShell = pkgs.mkShell {
        inherit buildInputs;
      };

      formatter = pkgs.alejandra;

      nixosModules = rec {
        bar-rs = default;
        default = {
        config,
        lib, pkgs
      }:
        let
          cfg = config.services.bar-rs;
        in {
          options = {
            services.bar-rs = {
              enable = lib.mkOption {
                type = lib.types.bool;
                default = false;
                example = true;
                description = lib.mdDoc ''
                  Enables bar-rs to run when your window manager starts
                '';
              };
              targets = lib.mkOption {
                type = lib.types.list;
                default = ["graphical-session.target"];
                example = ["hyprland-session.target"];
              };
            };
          };

          config = lib.mkIf cfg.enable {
            environment.systemPackages = [pkgs.bar-rs];
            systemd = {
              user = {
                services.bar-rs = {
                  wantedBy = cfg.targets;
                  script = lib.getExe pkgs.bar-rs.packages.${system}.default;
                  reloadIfChanged = true;
                };
              };
            };
          };
        };
      };
    });
}
