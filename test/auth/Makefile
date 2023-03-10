DIR=certs
PROTOCOL=x509
KEY_SIZE=4096
VALIDITY_DAYS=9999

CA_KEY=$(DIR)/sample-ca-key.pem
CA_CRT=$(DIR)/sample-ca-crt.pem

VMD_SERVER_KEY=$(DIR)/sample-vmd-server-key.pem
VMD_SERVER_CSR=$(DIR)/sample-vmd-server-csr.pem
VMD_SERVER_CRT=$(DIR)/sample-vmd-server-crt.pem

VMD_CLIENT_KEY=$(DIR)/sample-vmd-client-key.pem
VMD_CLIENT_CSR=$(DIR)/sample-vmd-client-csr.pem
VMD_CLIENT_CRT=$(DIR)/sample-vmd-client-crt.pem

COUNTRY=FI
STATE=Uusimaa
LOCALITY=Helsinki
ORGANIZATION=VMD
ORGANIZATION_UNIT=VMD

CA_COMMON_NAME=vmd-ca
VMD_SERVER_COMMON_NAME=vmd-server
VMD_CLIENT_COMMON_NAME=vmd-client

PASSWORD=1234
CA_SUBJECT=/C=$(COUNTRY)/ST=$(STATE)/L=$(LOCALITY)/O=$(ORGANIZATION)/OU=$(ORGANIZATION_UNIT)/CN=$(CA_COMMON_NAME)
SERVER_SUBJECT=/C=$(COUNTRY)/ST=$(STATE)/L=$(LOCALITY)/O=$(ORGANIZATION)/OU=$(ORGANIZATION_UNIT)/CN=$(VMD_SERVER_COMMON_NAME)
CLIENT_SUBJECT=/C=$(COUNTRY)/ST=$(STATE)/L=$(LOCALITY)/O=$(ORGANIZATION)/OU=$(ORGANIZATION_UNIT)/CN=$(VMD_CLIENT_COMMON_NAME)

all: ca server client

ca: $(DIR) $(CA_CRT)

server: $(CA_CRT)
	@echo "Generating vmd-server key and certificate signing request"
	@openssl genrsa -out $(VMD_SERVER_KEY) $(KEY_SIZE)
	@openssl req -new				\
		-key $(VMD_SERVER_KEY)		\
		-out $(VMD_SERVER_CSR)		\
		-passin pass:$(PASSWORD)	\
		-subj "$(SERVER_SUBJECT)"
	@openssl $(PROTOCOL) -req				\
		-days $(VALIDITY_DAYS)				\
		-in $(VMD_SERVER_CSR)				\
		-CA $(CA_CRT)						\
		-CAkey $(CA_KEY) -CAcreateserial	\
		-passin pass:$(PASSWORD)			\
		-out $(VMD_SERVER_CRT)
	@openssl verify -CAfile $(CA_CRT) $(VMD_SERVER_CRT)

client: $(CA_CRT)
	@echo "Generating client key and certificate signing request"
	@openssl genrsa -out $(VMD_CLIENT_KEY) $(KEY_SIZE)
	@openssl req -new				\
		-key $(VMD_CLIENT_KEY)		\
		-out $(VMD_CLIENT_CSR)		\
		-passin pass:$(PASSWORD)	\
		-subj "$(CLIENT_SUBJECT)"
	@openssl $(PROTOCOL) -req				\
		-days $(VALIDITY_DAYS)				\
		-in $(VMD_CLIENT_CSR)				\
		-CA $(CA_CRT)						\
		-CAkey $(CA_KEY) -CAcreateserial	\
		-passin pass:$(PASSWORD)			\
		-out $(VMD_CLIENT_CRT)
	@openssl verify -CAfile $(CA_CRT) $(VMD_CLIENT_CRT)

$(CA_CRT):
	@echo "Generating CA key and certificate"
	@openssl req -new -$(PROTOCOL)	\
		-days $(VALIDITY_DAYS)		\
		-keyout $(CA_KEY)			\
		-out $(CA_CRT)				\
		-passout pass:$(PASSWORD)	\
		-subj "$(CA_SUBJECT)"

$(DIR):
	mkdir -p $(DIR)

clean:
	rm -rf $(DIR)

fclean: clean

re: fclean all

.PHONY: all ca server client clean fclean re
