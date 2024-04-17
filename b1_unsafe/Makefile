all: run

build:
	RUSTFLAGS="-L ../libffi" cargo build

run: build
	DYLD_LIBRARY_PATH=../libffi ./target/debug/b1_unsafe
