{
  lib,
  stdenv,
  openapi-generator-cli,
}: let
  api-type = "rust-client";
in
  stdenv.mkDerivation {
    name = "vmd-${api-type}-api-src";
    version = "0.1.0";
    src = ./openapi.yaml;
    nativeBuildInputs = [
      openapi-generator-cli
    ];

    openapiGeneratorExtraArgs = [
      "--additional-properties=supportAsync=true"
      "--additional-properties=packageName=vmd-${api-type}-api"
    ];

    dontUnpack = true;
    buildPhase = ''
      ${openapi-generator-cli}/bin/openapi-generator-cli \
      generate \
      -g rust \
      -i $src \
      -o $out \
      $openapiGeneratorExtraArgs
    '';
  }
