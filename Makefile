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

# Build

all: generate-server-api generate-client-api build

generate-server-api: $(RUST_SERVER_API)

$(RUST_SERVER_API):
	openapi-generator-cli generate \
		-g rust-server \
		-i $(OPENAPI) \
		-o $(RUST_SERVER_API) \
		--additional-properties=packageName=vmd_rust_server_api

generate-client-api: $(RUST_CLIENT_API)

$(RUST_CLIENT_API):
	openapi-generator-cli generate \
		-g rust \
		-i $(OPENAPI) \
		-o $(RUST_CLIENT_API) \
		--additional-properties=supportAsync=true \
		--additional-properties=packageName=vmd_rust_client_api

build:
	cargo build --release

clean:
	cargo clean

fclean: clean
	make fclean -C test/auth
	rm -rf $(RUST_SERVER_API)
	rm -rf $(RUST_CLIENT_API)
	rm openapitools.json

re: fclean all

# Test

test: all generate-sample-certs test-server-up

test-server-up: all
	cargo run -p vmd_server --release -- \
		--addr localhost \
		--port $(PORT) \
		--cacert $(CA_CERT) \
		--cert $(SERVER_CERT) \
		--key $(SERVER_KEY)

regenerate-sample-certs:
	make fclean -C test/auth
	make -C test/auth

generate-sample-certs: $(CERTS)

$(CERTS):
	make -C test/auth

test-client:
	cargo run -p mtls-client -- \
		list \
		--port $(PORT) \
		--cacert $(CA_CERT) \
		--cert $(CLIENT_CHAIN) \
		--key $(CLIENT_KEY) \
		--hostname localhost

test-curl-client:
	curl --tlsv1.3 \
		--tls-max 1.3 \
		--cert $(CLIENT_CERT) \
		--key $(CLIENT_KEY) \
		--cacert $(CA_CERT) \
		--insecure \
		--verbose \
		https://localhost:$(PORT)/api/v1/vm/list/

test-wget-client:
	wget \
		--ca-cert=$(CA_CERT) \
		--certificate=$(CLIENT_CERT) \
     	--private-key=$(CLIENT_KEY) \
		https://localhost:$(PORT)/api/v1/vm/list

test-openssl-client:
	openssl s_client \
		-CAfile $(CA_CERT) \
		-cert $(CLIENT_CERT) \
		-key $(CLIENT_KEY) \
		-port $(PORT) \
		-connect localhost \

.PHONY: all generate-api build clean fclean re test generate-sample-certs test-curl-server
