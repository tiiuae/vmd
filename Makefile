.ONESHELL:

all: build

generate-sample-certs:
	make -C test/auth

test-curl-server:
	curl --cert test/auth/certs/sample-vmd-client-crt.pem	\
		--key test/auth/certs/sample-vmd-client-key.pem		\
		--cacert test/auth/certs/sample-ca-crt.pem			\
		http://localhost:8000/kvm_api_version

build:
	cargo build --release

clean:
	cargo clean

fclean: clean
	make fclean -C test/auth

re: fclean all

.PHONY: all create-test-certs build clean fclean re
