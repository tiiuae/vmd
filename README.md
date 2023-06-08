# VMD

Consist of a server and a client program for managing virtual machines ona Ghaf host.

# Build

Build the server using nix:

```bash
# Change target accordingly

nix build .#packages.x86_64-linux.vmd-server
```

Build the client using nix:

```bash
# Change target accordingly

nix build .#packages.x86_64-linux.vmd-client
```

# Running

1. Create certificates

```bash
make -C test/auth
```

2. Start server

```bash
nix run .#packages.x86_64-linux.vmd-server -- \
    --hostname localhost \
    --port 8080 \
    --cacert ./test/auth/certs/sample-ca-crt.pem \
    --cert ./test/auth/certs/sample-vmd-server-crt.pem \
    --key ./test/auth/certs/sample-vmd-server-key.pem
```

3. Run client

```bash
nix run .#packages.x86_64-linux.vmd-client -- \
	--hostname localhost \
    --port 8080 \
    --cacert test/auth/certs/sample-ca-crt.pem \
    --cert test/auth/certs/sample-vmd-client-chain.pem \
    --key test/auth/certs/sample-vmd-client-key.pem \
    --output yaml \
    list
```
