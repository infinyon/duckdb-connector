ARCH=$(shell uname -m)
NAME=duckdb-sink
RELEASE=release
DUCKDB_VER=v0.8.1
DUCKDB_LIB=duckdb_lib
LIB_NAME=$(if $(findstring arm64,$(ARCH)),libduckdb.dylib,libduckdb.so)
TARGET_FLAG=$(if $(TARGET),--target $(TARGET),)
FULL_LIB_NAME=$(DUCKDB_LIB_DIR)/$(LIB_NAME)
CDK=$(HOME)/.fluvio/bin/cdk

# We need to use dynamlic linking for OSX
export DUCKDB_LIB_DIR=$(PWD)/$(DUCKDB_LIB)/$(DUCKDB_VER)
#export LD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)
export DYLD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)

check-fmt:
	cargo fmt -- --check

test:
	echo "cargo test"

clippy:
	cargo clippy -- -D warnings


build: 
	$(CDK) build --release $(RELEASE) $(TARGET_FLAG)

# building as dynamic library, not used now
build_dyn: $(FULL_LIB_NAME)
	$(CDK) build --release $(RELEASE) $(TARGET_FLAG)

cross_build: $(FULL_LIB_NAME)
	cross build --release $(TARGET_FLAG)

zig_build: $(FULL_LIB_NAME)
	cargo zigbuild --release $(TARGET_FLAG)

test_md: 
	$(CDK) test  --release $(RELEASE)  --config test/duckdb-md.yaml --secrets .env $(TARGET_FLAG)

# build and test using dynamic library
test_md_dyn: $(FULL_LIB_NAME)
	$(CDK) test  --release $(RELEASE)  --config test/duckdb-md.yaml --secrets .env $(TARGET_FLAG)


test_local:
	$(CDK) test  --release $(RELEASE)  --config test/duckdb-local.yaml $(TARGET_FLAG)


$(DUCKDB_LIB_DIR)/libduckdb.dylib:
	gh release download $(DUCKDB_VER) --repo duckdb/duckdb --skip-existing -p libduckdb-osx-universal.zip -D $(DUCKDB_LIB_DIR)
	cd $(DUCKDB_LIB_DIR); unzip -n libduckdb-osx-universal.zip 

$(DUCKDB_LIB_DIR)/libduckdb.so:
	gh release download $(DUCKDB_VER) --repo duckdb/duckdb --skip-existing -p libduckdb-linux-aarch64.zip -D $(DUCKDB_LIB_DIR)
	cd $(DUCKDB_LIB_DIR); unzip -n libduckdb-linux-aarch64.zip

clean:
	cargo clean
	rm -rf $(DUCKDB_LIB)

.EXPORT_ALL_VARIABLES:
DUCKDB_STATIC=1
