all: run

build:
	RUSTFLAGS="-L ../libffi" cargo build

run: build
	LD_LIBRARY_PATH=../libffi ./target/debug/b1_unsafe
