build-web:
	CC=emcc wasm-pack build --release --out-dir ./pkg/web --target web

build-bundler:
	CC=emcc wasm-pack build --release --out-dir ./pkg/bundler --target bundler

build-nodejs:
	CC=emcc wasm-pack build --release --out-dir ./pkg/node --target nodejs

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd

publish-node:
	wasm-pack build --release --target nodejs
	wasm-pack publish ./pkg/node

publish-web:
	wasm-pack build --release --target web
	sed -i "s/bsv-wasm/bsv-wasm-web/" ./pkg/web/package.json
	wasm-pack publish ./pkg/web

publish-bundler:
	wasm-pack build --release --target bundler
	sed -i "s/bsv-wasm/bsv-wasm-bundler/" ./pkg/bundler/package.json
	wasm-pack publish ./pkg/bundler
