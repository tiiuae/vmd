{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
	nativeBuildInputs = with pkgs; [
		gnumake
		pkg-config
		openapi-generator-cli
		openssl.dev
	];
	buildInputs = with pkgs; [
		rustc
		cargo
	];
}