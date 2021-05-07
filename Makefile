build-web:
	CC=emcc wasm-pack build --release --target web

build-nodejs:
	CC=emcc wasm-pack build --release --target nodejs

build-deno:
	CC=emcc wasm-pack build --release --target deno

build:
	CC=emcc cargo web build --target=wasm32-unknown-unknown

publish-node:
	wasm-pack build --release --target nodejs
	wasm-pack publish ./pkg

publish-web:
	wasm-pack build --release --target web
	sed -i '' "s/bsv-wasm/bsv-wasm-web/" ./pkg/package.json
	wasm-pack publish ./pkg