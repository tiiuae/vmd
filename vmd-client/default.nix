{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  vmd-rust-client-api,
}:
rustPlatform.buildRustPackage {
  pname = "vmd-client";
  version = "0.1.0";

  nativeBuildInputs = [pkg-config];

  buildInputs = [vmd-rust-client-api openssl];

  postPatch = ''
    mkdir -pv ../vmd-api
    ln -s ${vmd-rust-client-api} ../vmd-api/rust-client
  '';

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
