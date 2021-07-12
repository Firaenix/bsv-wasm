build-web:
	wasm-pack build --release --out-dir ./pkg/web --target web -- -j 1

build-bundler:
	wasm-pack build --release --out-dir ./pkg/bundler --target bundler -- -j 1

build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs -- -j 1

build-wasm:
	make build-web ; make build-bundler ; make build-nodejs -- -j 1

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd

publish-node:
	wasm-pack build --release --target nodejs -- -j 1
	wasm-pack publish ./pkg

publish-web:
	wasm-pack build --release --target web -- -j 1
	sed -i "s/bsv-wasm/bsv-wasm-web/" ./pkg/package.json
	wasm-pack publish ./pkg

publish-bundler:
	wasm-pack build --release --target bundler -- -j 1
	sed -i "s/bsv-wasm/bsv-wasm-bundler/" ./pkg/package.json
	wasm-pack publish ./pkg

create-isomorphic:
	rollup pkg/node/bsv_wasm.js --format umd --name bsv_wasm_iso --file pkg/node/bsv_wasm_iso.js