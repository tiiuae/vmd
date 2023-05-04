let
  pkgs = import <nixpkgs> {};
in
pkgs.rustPlatform.buildRustPackage rec {
  pname = "vmd";
  version = "0.0.1";

  src = ./.;

  buildInputs = with pkgs; [
    pkg-config
    openssl
    openssl.dev
  ];

  cargoLock.lockFile = ./Cargo.lock;
  PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
}