{
  config,
  lib,
  pkgs,
  ...
}:
with lib; let
  cfg = config.services.vmd-server;
  defaultUser = "vmd-server";
  defaultGroup = "vmd";
in {
  options.services.vmd-server = {
    enable = mkEnableOption "vmd-server";

    package = mkOption {
      type = types.package;
      default = pkgs.callPackage ../vmd-server {
        vmd-rust-server-api = pkgs.callPackage ../vmd-api/rust-server.nix {};
      };
      description = mdDoc "The package to use for the vmd-server binary.";
    };

    generateKeys = mkEnableOption "generate keys during the boot";

    user = mkOption {
      type = types.str;
      default = defaultUser;
      example = "vmd-server";
      description = mdDoc ''
        The user for the service. If left as the default value this user will
        automatically be created.
      '';
    };

    group = mkOption {
      type = types.str;
      default = defaultGroup;
      example = "vmd";
      description = mdDoc ''
        The group for the service. If left as the default value this group will
        automatically be created.
      '';
    };

    bindAddress = mkOption {
      type = types.str;
      default = "localhost";
      description = mdDoc "vmd-server listening address.";
    };

    port = mkOption {
      type = types.port;
      default = 9876;
      description = mdDoc "vmd-server listening port.";
    };

    caCertPath = mkOption {
      type = types.path;
      default = "/run/vmd/sample-ca-cert.pem";
      description = mdDoc "vmd-server certificate authority file.";
    };

    tlsCertPath = mkOption {
      type = types.path;
      default = "/run/vmd/sample-vmd-server-crt.pem";
      description = mdDoc "vmd-server certificate file.";
    };

    tlsKeyPath = mkOption {
      type = types.path;
      default = "/run/vmd/sample-vmd-server-key.pem";
      description = mdDoc "vmd-server private key file.";
    };
  };

  config = mkIf cfg.enable {
    users = {
      users = mkIf (cfg.user == defaultUser) {
        vmd-server = {
          group = cfg.group;
          isSystemUser = true;
        };
      };
      groups = mkIf (cfg.group == defaultGroup) {
        vmd = {};
      };
    };
    systemd.services."vmd-keygen" = let
      keygenScript = pkgs.callPackage ./vmd-keygen.nix {
        vmd-user = cfg.user;
        vmd-group = cfg.group;
        inherit (cfg) caCertPath tlsCertPath tlsKeyPath;
      };
    in
      mkIf cfg.generateKeys {
        enable = true;
        description = "Generate keys for Virtual Machine Daemon";
        documentation = ["https://github.com/tiiuae/vmd"];
        path = [keygenScript];
        before = ["vmd-server.service"];
        serviceConfig = {
          Type = "oneshot";
          RemainAfterExit = true;
          StandardOutput = "journal";
          StandardError = "journal";
          ExecStart = "${keygenScript}/bin/vmd-keygen";
        };
      };
    systemd.services."vmd-server" = {
      enable = true;
      wants = mkIf cfg.generateKeys ["vmd-keygen.service"];
      wantedBy = ["multi-user.target"];
      description = "Virtual Machine Daemon";
      documentation = ["https://github.com/tiiuae/vmd"];
      path = [cfg.package];
      serviceConfig = {
        User = cfg.user;
        Group = cfg.group;
        Type = "simple";
        StandardOutput = "journal";
        StandardError = "journal";
        Environment = "RUST_LOG=trace";
        ExecStart = lib.concatStringsSep " " [
          "${cfg.package}/bin/vmd-server"
          "--hostname"
          cfg.bindAddress
          "--port"
          (builtins.toString cfg.port)
          "--cacert"
          cfg.caCertPath
          "--cert"
          cfg.tlsCertPath
          "--key"
          cfg.tlsKeyPath
        ];
      };
    };
  };
}
