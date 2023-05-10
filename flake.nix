{
  description = "Virtual Machine Daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    crane = {
		url = "github:ipetkov/crane";
		inputs.nixpkgs.follows = "nixpkgs";
		inputs.flake-utils.follows = "flake-utils";
    };

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, crane, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
		let
			pkgs = import nixpkgs {
				inherit system;
			};

			craneLib = crane.lib.${system};
			package = craneLib.buildPackage {
				pname = "vmd";
				version = "0.1.0";
				src = craneLib.cleanCargoSource (craneLib.path ./.);

				nativeBuildInputs = with pkgs; [
					gnumake
					pkg-config
					openapi-generator-cli
					openssl.dev
					rustc
					cargo
				];

				PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
			};
		in {
			checks = {
				inherit package;
			};

			packages.default = package;

			apps.default = flake-utils.lib.mkApp {
				drv = package;
			};

			devShells.default = pkgs.mkShell {
				inputsFrom = builtins.attrValues self.checks.${system};

				nativeBuildInputs = with pkgs; [
					gnumake
					pkg-config
					openapi-generator-cli
					openssl.dev
					rustc
					cargo
				];

				PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
			};
		}
	);
}

