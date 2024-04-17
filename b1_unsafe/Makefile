LIBPATH = LD_LIBRARY_PATH=../libffi
ifeq ($(shell uname -s), Darwin)
	LIBPATH = DYLD_LIBRARY_PATH=../libffi
endif

all: run

build:
	RUSTFLAGS="-L ../libffi" cargo build

run: build
	$(LIBPATH) ./target/debug/b1_unsafe
