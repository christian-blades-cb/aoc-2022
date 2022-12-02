{
  description = "Advent of Code 2022";
  
  inputs.flake-utils.url = "github:numtide/flake-utils";
  inputs.fenix = {
    url = "github:nix-community/fenix";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  
  outputs = { self, nixpkgs, flake-utils, fenix }: 
    flake-utils.lib.eachDefaultSystem
      ( system:
        let
          pkgs = nixpkgs.legacyPackages.${system}.appendOverlays [ fenix.overlays.default ];
        in
          {
            devShells.default = import ./shell.nix { inherit pkgs; };
          }
      );
}
