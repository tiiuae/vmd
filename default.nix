let
	pkgs = import <nixpkgs> {};
in pkgs.rustPlatform.buildRustPackage rec {
	pname = "vmd";
	version = "0.0.1";
	cargoLock.lockFile = ./Cargo.lock;
	PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

	src = ./.;

	nativeBuildInputs = with pkgs; [
		pkg-config
		openssl.dev
		openapi-generator-cli
		gnumake
	];

	configurePhase = ''
		make generate-server-api;
		make generate-client-api;
	'';
}