all: create-test-certs build

create-test-certs:
	make -C auth

build:
	cargo build --release

clean:
	cargo clean

fclean: clean
	make fclean -C auth

re: fclean all

.PHONY: all create-test-certs build clean fclean re
