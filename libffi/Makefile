all: libffi.so

ffi.o: ffi.c
	cc -c -fPIC -o ffi.o ffi.c

libffi.so: ffi.o
	cc -shared -fPIC -o libffi.so ffi.o

clean:
	-rm -rf *.so *.dylib *.o
