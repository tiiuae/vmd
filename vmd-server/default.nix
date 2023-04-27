{
  lib,
  rustPlatform,
  pkg-config,
  openssl,
  vmd-rust-server-api,
}:
rustPlatform.buildRustPackage {
  pname = "vmd-server";
  version = "0.1.0";

  nativeBuildInputs = [pkg-config];

  buildInputs = [vmd-rust-server-api openssl];

  postPatch = ''
    mkdir -pv ../vmd-api
    ln -s ${vmd-rust-server-api} ../vmd-api/rust-server
  '';

  src = ./.;

  cargoLock = {
    lockFile = ./Cargo.lock;
  };
}
