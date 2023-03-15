{
  description = "A simple flake for mock-okx-api";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    cargo2nix.url = "github:cargo2nix/cargo2nix/release-0.11.0";
  };

  outputs = { self, nixpkgs, flake-utils, cargo2nix }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        rustBuilder = import cargo2nix { pkgs = pkgs; };
        rustPkgs = rustBuilder.makePackageSet {
          rustVersion = "1.61.0";
          packageFun = import ./. /Cargo.nix;
        };
      in rec {
        packages = {
          mock-okx-api = (rustPkgs.workspace.mock-okx-api {}).bin;
          default = packages.mock-okx-api;
        };
      }
    );
}
