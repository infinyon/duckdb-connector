DUCKDB_VER=v0.8.1
DUCKDB_LIB=duckdb_lib
# We need to use dynamlic linking to in order to invoke motherduck
export DUCKDB_LIB_DIR=$(PWD)/$(DUCKDB_LIB)/$(DUCKDB_VER)

build: $(DUCKDB_LIB_DIR)/libduckdb.dylib
	cdk build -p duckdb-sink --release release

test_md: $(DUCKDB_LIB_DIR)/libduckdb.dylib
	DYLD_LIBRARY_PATH=$(DUCKDB_LIB_DIR) cdk test  --release release  -p duckdb-sink --config duckdb-md.yaml --secrets .env

test_local:
	cdk test  --release release  -p duckdb-sink --config duckdb-local.yaml


$(DUCKDB_LIB_DIR)/libduckdb.dylib:
	gh release download $(DUCKDB_VER) --repo duckdb/duckdb --clobber -p libduckdb-osx-universal.zip -D $(DUCKDB_LIB_DIR)
	pushd $(DUCKDB_LIB_DIR); unzip libduckdb-osx-universal.zip; popd

clean:
	cargo clean
	rm -rf $(DUCKDB_LIB)