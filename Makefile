build-web:
	wasm-pack build --release --out-dir ./pkg/web --target web

build-bundler:
	wasm-pack build --release --out-dir ./pkg/bundler --target bundler

build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs

build-wasm:
	make build-web ; make build-bundler ; make build-nodejs

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd

publish-node:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs
	wasm-pack publish ./pkg/node

publish-web:
	wasm-pack build --out-dir ./pkg/web--release --target web
	sed -i "s/bsv-wasm/bsv-wasm-web/" ./pkg/web/package.json
	wasm-pack publish ./pkg/web

publish-bundler:
	wasm-pack build --out-dir ./pkg/bundler --release --target bundler
	sed -i "s/bsv-wasm/bsv-wasm-bundler/" ./pkg/bundler/package.json
	wasm-pack publish ./pkg/bundler

create-isomorphic:
	rollup pkg/node/bsv_wasm.js --format umd --name bsv_wasm_iso --file pkg/node/bsv_wasm_iso.js