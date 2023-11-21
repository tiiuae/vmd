{
  bash,
  openssl,
  writeShellScriptBin,
  vmd-user,
  vmd-group,
  caCertPath,
  tlsCertPath,
  tlsKeyPath,
}: let
  dir = "/run/vmd";
  protocol = "x509";
  cipherSuite = "TLS_AES_256_GCM_SHA384";
  keySize = 4096;
  validityDays = 9999;

  caKey = "${dir}/sample-ca-key.pem";
  caCrt = "${caCertPath}";

  vmdServerKey = "${tlsKeyPath}";
  vmdServerCsr = "${dir}/sample-vmd-server-csr.pem";
  vmdServerCrt = "${tlsCertPath}";
  vmdServerChain = "${dir}/sample-vmd-server-chain.pem";

  vmdClientKey = "${dir}/sample-vmd-client-key.pem";
  vmdClientCsr = "${dir}/sample-vmd-client-csr.pem";
  vmdClientCrt = "${dir}/sample-vmd-client-crt.pem";
  vmdClientChain = "${dir}/sample-vmd-client-chain.pem";

  country = "FI";
  state = "Uusimaa";
  locality = "Helsinki";
  organization = "VMD";
  organizationUnit = "VMD";

  caCommonName = "vmd-ca";
  # TLS requires URL's to match so we use localhost
  vmdServerCommonName = "localhost";
  vmdClientCommonName = "vmd-client";

  alt = "DNS:localhost";

  password = "1234";
  caSubject = "/C=${country}/ST=${state}/L=${locality}/O=${organization}/OU=${organizationUnit}/CN=${caCommonName}";
  serverSubject = "/C=${country}/ST=${state}/L=${locality}/O=${organization}/OU=${organizationUnit}/CN=${vmdServerCommonName}";
  clientSubject = "/C=${country}/ST=${state}/L=${locality}/O=${organization}/OU=${organizationUnit}/CN=${vmdClientCommonName}";

  ossl = "${openssl}/bin/openssl";
in
  writeShellScriptBin "vmd-keygen" ''
    set -euo pipefail

    mkdir -p ${dir}
    chmod 550 ${dir}
    umask 007

    echo "Generating CA key and certificate"
    ${ossl} req -new -${protocol} \
      -days ${builtins.toString validityDays} \
      -keyout ${caKey} \
      -out ${caCrt} \
      -passout pass:${password} \
      -subj "${caSubject}"

    echo "Generating vmd-server key and certificate signing request"
    ${ossl} genrsa -out ${vmdServerKey} ${builtins.toString keySize}
    ${ossl} req -new \
        -key ${vmdServerKey} \
        -out ${vmdServerCsr} \
        -passin pass:${password} \
        -subj "${serverSubject}"
    ${bash}/bin/bash -c '${ossl} ${protocol} -req \
        -days ${builtins.toString validityDays} \
        -in ${vmdServerCsr} \
        -CA ${caCrt} \
        -CAkey ${caKey} -CAcreateserial \
        -passin pass:${password} \
        -out ${vmdServerCrt} \
        -extfile <(printf "subjectAltName=${alt}")'
    ${ossl} verify -CAfile ${caCrt} ${vmdServerCrt}
    cat ${vmdServerCrt} ${vmdServerKey} > ${vmdServerChain}

    echo "Generating client key and certificate signing request"
    ${ossl} genrsa -out ${vmdClientKey} ${builtins.toString keySize}
    ${ossl} req -new \
        -key ${vmdClientKey} \
        -out ${vmdClientCsr} \
        -passin pass:${password} \
        -subj "${clientSubject}"
    ${bash}/bin/bash -c '${ossl} ${protocol} -req \
        -days ${builtins.toString validityDays} \
        -in ${vmdClientCsr} \
        -CA ${caCrt} \
        -CAkey ${caKey} -CAcreateserial \
        -passin pass:${password} \
        -out ${vmdClientCrt} \
        -extfile <(printf "subjectAltName=${alt}")'
    ${ossl} verify -CAfile ${caCrt} ${vmdClientCrt}
    cat ${vmdClientCrt} ${vmdClientKey} > ${vmdClientChain}

    # TODO: File permissions need additional tweaking for security
    chown -R ${vmd-user}:${vmd-group} ${dir} ${caCertPath} ${tlsCertPath} ${tlsKeyPath}
  ''
