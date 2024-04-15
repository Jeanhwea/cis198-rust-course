all: libffi.dylib


ffi.o: ffi.c
	cc -c -fPIC -o ffi.o ffi.c

libffi.dylib: ffi.o
	cc -shared -fPIC -o libffi.dylib ffi.o
