build-web:
	CC=emcc wasm-pack build --release --target web

build-bundler:
	CC=emcc wasm-pack build --release --target bundler

build-nodejs:
	CC=emcc wasm-pack build --release --target nodejs

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd

publish-node:
	wasm-pack build --release --target nodejs
	wasm-pack publish ./pkg

publish-web:
	wasm-pack build --release --target web
	sed -i "s/bsv-wasm/bsv-wasm-web/" ./pkg/package.json
	wasm-pack publish ./pkg
