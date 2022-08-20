all: test

build: src/gen/ffi.rs src/gen/ffi.cc
	cargo build

src/gen/ffi.cc: tpl/gvars.py tpl/ffi.cc
	./tools/j2rxx.py -o $@ -g $^

src/gen/ffi.rs: tpl/gvars.py tpl/ffi.rs
	./tools/j2rxx.py -o $@ -g $^

test: build
	cargo test -- --nocapture
