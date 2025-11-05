{ pkgs ? import <nixpkgs> {} }:

let
  libs = with pkgs; [
    libGL
    libxkbcommon
    wayland
    cargo
    pkg-config 
    openssl
  ];
in
pkgs.mkShell {
  buildInputs = [ pkgs.cargo pkgs.rustc ] ++ libs;
}