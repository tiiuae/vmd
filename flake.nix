{
  description = "Virtual Machine Daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-23.05";

    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    ...
  }: let
    systems = with flake-utils.lib.system; [
      x86_64-linux
      aarch64-linux
    ];
  in
    flake-utils.lib.eachSystem systems (
      system: let
        pkgs = import nixpkgs {
          inherit system;
        };
      in {
        packages.vmd-rust-server-api = nixpkgs.legacyPackages.${system}.callPackage ./vmd-api/rust-server.nix {};
        packages.vmd-rust-client-api = nixpkgs.legacyPackages.${system}.callPackage ./vmd-api/rust-client.nix {};
        packages.vmd-server = nixpkgs.legacyPackages.${system}.callPackage ./vmd-server {
          vmd-rust-server-api = self.packages.${system}.vmd-rust-server-api;
        };
        packages.vmd-client = nixpkgs.legacyPackages.${system}.callPackage ./vmd-client {
          vmd-rust-client-api = self.packages.${system}.vmd-rust-client-api;
        };

        formatter = nixpkgs.legacyPackages.${system}.alejandra;
        devShells.default = nixpkgs.legacyPackages.${system}.mkShell {
          nativeBuildInputs = with pkgs; [ darwin.apple_sdk.frameworks.Security pkg-config openssl ];
        };
      }
    );
}
