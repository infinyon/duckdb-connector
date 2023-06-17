ARCH=$(shell uname -m)
DUCKDB_VER=v0.8.1
DUCKDB_LIB=duckdb_lib
LIB_NAME=$(if $(findstring arm64,$(ARCH)),libduckdb.dylib,libduckdb.so)
TARGET_FLAG=$(if $(findstring arm64,$(ARCH)),,--target aarch64-unknown-linux-musl)
FULL_LIB_NAME=$(DUCKDB_LIB_DIR)/$(LIB_NAME)

# We need to use dynamlic linking in order to invoke motherduck
export DUCKDB_LIB_DIR=$(PWD)/$(DUCKDB_LIB)/$(DUCKDB_VER)
export LD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)
export DYLD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)

build: $(FULL_LIB_NAME)
	cdk build -p duckdb-sink --release release $(TARGET_FLAG)

test_md: $(FULL_LIB_NAME)
	cdk test  --release release  -p duckdb-sink --config test/duckdb-md.yaml --secrets .env $(TARGET_FLAG)

test_local:
	cdk test  --release release  -p duckdb-sink --config duckdb-local.yaml


$(DUCKDB_LIB_DIR)/libduckdb.dylib:
	gh release download $(DUCKDB_VER) --repo duckdb/duckdb --skip-existing -p libduckdb-osx-universal.zip -D $(DUCKDB_LIB_DIR)
	cd $(DUCKDB_LIB_DIR); unzip -n libduckdb-osx-universal.zip 

$(DUCKDB_LIB_DIR)/libduckdb.so:
	gh release download $(DUCKDB_VER) --repo duckdb/duckdb --skip-existing -p libduckdb-linux-aarch64.zip -D $(DUCKDB_LIB_DIR)
	cd $(DUCKDB_LIB_DIR); unzip -n libduckdb-linux-aarch64.zip

clean:
	cargo clean
	rm -rf $(DUCKDB_LIB)