{
  description = "A libadwaita application in Rust";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        appDerivation = import ./default.nix { inherit pkgs; };
      in
      {
        defaultPackage = appDerivation;
      }
    );
}
