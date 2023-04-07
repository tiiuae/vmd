.ONESHELL:

OPENAPI=vmd-api/openapi.yaml
CERTS=test/auth/certs
RUST_API=vmd-api/rust

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
		--port 8000 \
		--ca test/auth/certs/sample-ca-crt.pem \
		--crt test/auth/certs/sample-vmd-server-crt.pem \
		--key test/auth/certs/sample-vmd-server-key.pem

generate-sample-certs: $(CERTS)

$(CERTS):
	make -C test/auth

test-curl-server:
	curl --cert test/auth/certs/sample-vmd-client-crt.pem \
		--key test/auth/certs/sample-vmd-client-key.pem \
		--cacert test/auth/certs/sample-ca-crt.pem \
		https://localhost:8000/ \
		--verbose

.PHONY: all generate-api build clean fclean re test generate-sample-certs test-curl-server
