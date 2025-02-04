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
    apps = forAllSystems (system: {
      iloc = let
        pkg = self.packages.${system}.iloc;
      in {
        type = "app";
        program = "${pkg}/bin/${pkg.pname}";
      };

      default = self.apps.${system}.iloc;
    });

    packages = forAllSystems (system: let
      pkgs = getPkgs system;
    in {
      iloc = pkgs.rustPlatform.buildRustPackage {
        pname = "iloc";
        version = "0.1.0";
        src = ./.;

        cargoLock.lockFile = ./Cargo.lock;
      };

      default = self.packages.${system}.iloc;
    });
  };
}
