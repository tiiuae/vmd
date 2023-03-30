.ONESHELL:

all: build

generate-sample-certs:
	make -C test/auth

generate-server-api:
	openapi-generator-cli generate \
		-g rust-server \
		-i vmd-api/openapi/openapi.yaml \
		-o vmd-api/server

test-curl-server:
	curl --cert test/auth/certs/sample-vmd-client-crt.pem \
		--key test/auth/certs/sample-vmd-client-key.pem \
		--cacert test/auth/certs/sample-ca-crt.pem \
		http://localhost:8000/

build:
	cargo build --release

clean:
	cargo clean

fclean: clean
	make fclean -C test/auth
	rm -rf vmd-api/server
	rm openapitools.json

re: fclean all

.PHONY: all create-test-certs build clean fclean re
