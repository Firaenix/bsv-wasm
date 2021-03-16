build-web:
	CC=emcc wasm-pack build --release --target web

build-nodejs:
	CC=emcc wasm-pack build --release --target nodejs

build-deno:
	CC=emcc wasm-pack build --release --target deno

build:
	CC=emcc cargo web build --target=wasm32-unknown-unknown