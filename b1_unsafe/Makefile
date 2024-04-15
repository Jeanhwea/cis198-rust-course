all: run

build:
	RUST_FLAGS="-L ../libffi" cargo build

run: build
	LD_LIBRARY_PATH=. ./target/debug/b1_unsafe
