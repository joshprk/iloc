{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
    ...
  }: let
    lib = nixpkgs.lib;
    forAllSystems = lib.genAttrs lib.systems.flakeExposed;
    getPkgs = system: import nixpkgs {
      inherit system;
      overlays = [
        (import rust-overlay)
      ];
    };
  in {
    packages = forAllSystems (system: let
      pkgs = getPkgs system;
    in {
      iloc = pkgs.stdenv.mkDerivation {
        pname = "iloc";
        version = "0.1.0";

        buildInputs = with pkgs; [
          rust-bin.stable.latest.default
        ];
      };

      default = self.packages.${system}.iloc;
    });
  };
}
