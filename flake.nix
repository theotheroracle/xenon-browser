{
  description = "xenon-browser";

  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:theotheroracle/nixpkgs";
  };
  
  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let 
          pkgs = nixpkgs.legacyPackages.${system};
        in
        {
          devShells.default = import ./shell.nix { inherit pkgs; };
		    packages.default = pkgs.callPackage ./default.nix { };
        }
      );
}
