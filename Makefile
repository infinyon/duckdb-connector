ARCH=$(shell uname -m)
DUCKDB_VER=v0.8.1
NAME=duckdb-sink
RELEASE=release

TARGET_FLAG=$(if $(TARGET),--target $(TARGET),)


build: 
	cargo build --profile $(RELEASE) $(TARGET_FLAG)

test_bin:
	./target/$(TARGET)/$(RELEASE)/$(NAME) --config test/duckdb-md.yaml --secrets .env

# to run as linux gnu: make test_md TARGET=aarch64-unknown-linux-gnu
test_md: $(FULL_LIB_NAME)
	cdk test  --release release  -p duckdb-sink --config test/duckdb-md.yaml --secrets .env $(TARGET_FLAG)

test_local:
	cdk test  --release release  -p duckdb-sink --config duckdb-local.yaml


# these targets are used for dynamic linking. for now it is unneeded, in order to use uncomment this out
#DUCKDB_LIB=duckdb_lib
#LIB_NAME=$(if $(findstring arm64,$(ARCH)),libduckdb.dylib,libduckdb.so)
#FULL_LIB_NAME=$(DUCKDB_LIB_DIR)/$(LIB_NAME)
#export DUCKDB_LIB_DIR=$(PWD)/$(DUCKDB_LIB)/$(DUCKDB_VER)
#export LD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)
#export DYLD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)
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
#DUCKDB_LIB_DIR=$(PWD)/$(DUCKDB_LIB)/$(DUCKDB_VER)
#DUCKDB_STATIC=1
#LD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)
#DYLD_LIBRARY_PATH=$(DUCKDB_LIB_DIR)
FLUVIO_BUILD_ZIG ?= zig
FLUVIO_BUILD_LLD ?= lld
# used by CC crates to find CC which is replaced by zig
CC_aarch64_unknown_linux_musl=$(PWD)/build-scripts/aarch64-linux-musl-zig-cc
CXX_aarch64_unknown_linux_musl=$(PWD)/build-scripts/aarch64-linux-musl-zig-cxx
#CC_x86_64_unknown_linux_musl=$(PWD)/build-scripts/x86_64-linux-musl-zig-cc
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=$(PWD)/build-scripts/ld.lld
#CC="zig cc -target aarch64-linux-musl"
#CXX="zig c++ -target aarch64-linux-musl"
#CARGO_TARGET_X86_64_UNKNOWN_LINUX_MUSL_LINKER=$(PWD)/build-scripts/ld.lld
#RUSTFLAGS="-C link-arg=-lstdc++"