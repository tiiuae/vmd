.ONESHELL:

OPENAPI=vmd-api/openapi.yaml
RUST_API=vmd-api/rust

CERTS=test/auth/certs
CLIENT_CERT=$(CERTS)/sample-vmd-client-crt.pem
CLIENT_KEY=$(CERTS)/sample-vmd-client-key.pem
SERVER_CERT=$(CERTS)/sample-vmd-server-crt.pem
SERVER_KEY=$(CERTS)/sample-vmd-server-key.pem
CA_CERT=$(CERTS)/sample-ca-crt.pem

# Build

all: generate-api build

generate-api: $(RUST_API)

$(RUST_API):
	openapi-generator-cli generate \
		-g rust-server \
		-i $(OPENAPI) \
		-o $(RUST_API) \
		--additional-properties=packageName=vmd_api

build:
	cargo build --release

clean:
	cargo clean

fclean: clean
	make fclean -C test/auth
	rm -rf $(RUST_API)
	rm openapitools.json

re: fclean all

# Test

test: generate-sample-certs

test-server-up: all
	cargo run -p vmd_server --release -- \
		--addr localhost \
		--port 6666 \
		--cacert $(CA_CERT) \
		--cert $(SERVER_CERT) \
		--key $(SERVER_KEY)

generate-sample-certs: $(CERTS)

$(CERTS):
	make -C test/auth

test-curl-server:
	curl --tlsv1.3 \
		--tls-max 1.3 \
		--cert $(CLIENT_CERT) \
		--key $(CLIENT_KEY) \
		--cacert $(CA_CERT) \
		--capath test/auth/certs \
		--verbose \
		https://localhost:6666/api/vi/vm/list

.PHONY: all generate-api build clean fclean re test generate-sample-certs test-curl-server
