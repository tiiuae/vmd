.ONESHELL:

OPENAPI=vmd-api/openapi.yaml
RUST_SERVER_API=vmd-api/rust-server
RUST_CLIENT_API=vmd-api/rust-client

CERTS=test/auth/certs
CLIENT_CERT=$(CERTS)/sample-vmd-client-crt.pem
CLIENT_KEY=$(CERTS)/sample-vmd-client-key.pem
CLIENT_CHAIN=$(CERTS)/sample-vmd-client-chain.pem
SERVER_CERT=$(CERTS)/sample-vmd-server-crt.pem
SERVER_KEY=$(CERTS)/sample-vmd-server-key.pem
CA_CERT=$(CERTS)/sample-ca-crt.pem

PORT=8080
HOSTNAME=localhost

# Generate necessary API bindings and make a release build.
all: generate-server-api build

# Generate a server API from the OpenAPI specification. The generated
# code is located in the `vmd-api/rust-server` directory and used by the
# vmd-server crate. In order to work on the server code or to run tests
# you need to run this command first. Generated code is not committed to
# the repository.
generate-server-api: $(RUST_SERVER_API)

$(RUST_SERVER_API):
	openapi-generator-cli generate \
		-g rust-server \
		-i $(OPENAPI) \
		-o $(RUST_SERVER_API) \
		--additional-properties=packageName=vmd_rust_server_api

# Generate a client API from the OpenAPI specification. The generated
# code is located in the `vmd-api/rust-client` directory and used only
# for testing purposes. In order to run tests you need to run this
# command first. Generated code is not committed to the repository.
generate-client-api: $(RUST_CLIENT_API)

$(RUST_CLIENT_API):
	openapi-generator-cli generate \
		-g rust \
		-i $(OPENAPI) \
		-o $(RUST_CLIENT_API) \
		--additional-properties=supportAsync=true \
		--additional-properties=packageName=vmd_rust_client_api

# Build `vmd` in release mode.
build:
	cargo build --release

# Build `vmd` in debug mode.
build-debug:
	cargo build

# Remove build artifacts.
clean:
	cargo clean

# Remove everything that is not part of source control.
fclean: clean
	make fclean -C test/auth
	rm -rf $(RUST_SERVER_API)
	rm -rf $(RUST_CLIENT_API)
	rm openapitools.json

# Rebuild everything.
re: fclean all

# Test

# Generate bindings and build code in debug mode for testing.
build-test: generate-sample-certs generate-client-api generate-server-api build-debug

# Run test suite.
test: build-test test-server-up
	echo "Coming soon..."

# Run test server.
test-server-up: build-test
	cargo run -p vmd_server --release -- \
		--hostname localhost \
		--port $(PORT) \
		--cacert $(CA_CERT) \
		--cert $(SERVER_CERT) \
		--key $(SERVER_KEY)

# Regenerate sample certificates even if they already exist.
regenerate-sample-certs:
	make fclean -C test/auth
	make -C test/auth

# Generate sample certificates if they don't exist.
generate-sample-certs: $(CERTS)

$(CERTS):
	make -C test/auth

# Simple test using custom client.
test-client:
	cargo run -p mtls-client -- \
		list \
		--port $(PORT) \
		--cacert $(CA_CERT) \
		--cert $(CLIENT_CHAIN) \
		--key $(CLIENT_KEY) \
		--hostname $(HOSTNAME)

test-curl-client:
	curl --tlsv1.3 \
		--tls-max 1.3 \
		--cert $(CLIENT_CERT) \
		--key $(CLIENT_KEY) \
		--cacert $(CA_CERT) \
		--insecure \
		--verbose \
		https://$(HOSTNAME):$(PORT)/api/v1/vm/list/

test-wget-client:
	wget \
		--ca-cert=$(CA_CERT) \
		--certificate=$(CLIENT_CERT) \
		--private-key=$(CLIENT_KEY) \
		https://$(HOSTNAME):$(PORT)/api/v1/vm/list

test-openssl-client:
	openssl s_client \
		-CAfile $(CA_CERT) \
		-cert $(CLIENT_CERT) \
		-key $(CLIENT_KEY) \
		-port $(PORT) \
		-connect $(HOSTNAME) \

.PHONY: all generate-server-api generate-client-api build build-debug clean fclean re test test-server-up regenerate-sample-certs generate-sample-certs test-client test-curl-client test-wget-client test-openssl-client
