build:
	wasm-pack build -t no-modules --release

.PHONY:	all

all:	build
