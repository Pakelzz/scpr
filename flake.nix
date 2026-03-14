{
  description = "Rust project";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";

    crane.url = "github:ipetkov/crane";

    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, crane, fenix }:
  let
    system = "x86_64-linux";

    pkgs = import nixpkgs {
      inherit system;
    };

    toolchain = import ./nix/toolchain.nix {
      inherit fenix system;
    };

    craneLib = import ./nix/crane.nix {
      inherit pkgs crane toolchain;
    };

    src = craneLib.cleanCargoSource ./.;

    cargoArtifacts = craneLib.buildDepsOnly {
      inherit src;
    };

    package = craneLib.buildPackage {
      inherit src cargoArtifacts;
      pname = "scpr";
      version = "0.1.0";
    };

  in {

    packages.${system}.default = package;

    overlays.default = final: prev: {
      scpr = package;
    };

    devShells.${system}.default =
      import ./nix/devshell.nix {
        inherit pkgs toolchain;
      };

  };
}