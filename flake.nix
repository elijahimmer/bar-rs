{
  description = "A GTK4 Wayland Status Bar";
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = {
    self,
    nixpkgs,
  }: let
    supportedSystems = ["x86_64-linux"];
    forAllSystems = nixpkgs.lib.genAttrs supportedSystems;
    pkgsFor = nixpkgs.legacyPackages;
  in {
    packages = forAllSystems (system: {
      default = pkgsFor.${system}.callPackage ./. {};
    });
    devShells = forAllSystems (system: {
      default = pkgsFor.${system}.mkShell {
        name = "nix";
        nativeBuildInputs = with pkgsFor.${system}; [
          cargo
          rustc
          pkg-config
        ];

        buildInputs = with pkgsFor.${system}; [
          glib
          gtk4
          gtk4-layer-shell
        ];

     };
    });

    formatter = forAllSystems (system: nixpkgs.legacyPackages.${system}.alejandra);
  };
}
