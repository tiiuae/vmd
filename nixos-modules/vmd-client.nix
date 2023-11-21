{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.programs.vmd-client;
  cfgS = config.services.vmd-server;
in {
  options.programs.vmd-client = {
    enable = mkEnableOption "vmd-client";

    package = mkOption {
      type = types.package;
      default =
        (pkgs.callPackage ../vmd-client {
          vmd-rust-client-api = pkgs.callPackage ../vmd-api/rust-client.nix {};
        })
        # TODO: Default config file path for vmd-client would be nicer than
        #       this hackery
        .overrideAttrs (prevAttrs: let
          vmdClientArgs = lib.concatStringsSep " " ([
              "--add-flags"
              "--hostname"
              "--add-flags"
              cfgS.bindAddress
              "--add-flags"
              "--port"
              "--add-flags"
              (builtins.toString cfgS.port)
              "--add-flags"
              "--cacert"
              "--add-flags"
              cfgS.caCertPath
            ]
            ++ lib.optionals cfgS.generateKeys
            [
              "--add-flags"
              "--cert"
              "--add-flags"
              "/run/vmd/sample-vmd-client-chain.pem"
              "--add-flags"
              "--key"
              "--add-flags"
              "/run/vmd/sample-vmd-client-key.pem"
            ]);
        in
          lib.optionalAttrs cfgS.enable {
            nativeBuildInputs = prevAttrs.nativeBuildInputs ++ [pkgs.makeWrapper];
            postFixup = ''
              wrapProgram $out/bin/vmd-client ${vmdClientArgs}
            '';
          });
      description = mdDoc "The package to use for the vmd-client binary.";
    };
  };

  config = mkIf cfg.enable {
    environment.systemPackages = [
      cfg.package
    ];
  };
}
