all: libffi.dylib

ffi.o: ffi.c
	cc -c -fPIC -o ffi.o ffi.c

libffi.so: ffi.o
	cc -shared -fPIC -o libffi.so ffi.o

libffi.dylib: ffi.o
	cc -shared -fPIC -o libffi.dylib ffi.o
