{ pkgs ? import <nixpkgs> {} }:

pkgs.mkShell {
  buildInputs = with pkgs; [
    pkg-config
    (fenix.fromToolchainFile { dir = ./.; sha256 = "sha256-bWDPQLhuWCAxyzaAYywJmPktYh81TOSV3+cQDgj3xVE="; })
  ];
}
