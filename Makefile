check-format:
	cargo fmt -- --check && cargo clippy -- -Dwarnings

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

test-wasm:
	wasm-pack test --node

test:
	cargo test

test-all:
	make test && make test-wasm && make test-node

publish-node:
	# make sure not to call make build-* because wasm-pack doesnt allow you to specify subdirectories.
	wasm-pack build --release --target nodejs
	wasm-pack publish ./pkg

publish-web:
	wasm-pack build --release --target web
	sed -i "s/bsv-wasm/bsv-wasm-web/" ./pkg/package.json
	wasm-pack publish ./pkg

publish-bundler:
	wasm-pack build --release --target bundler
	sed -i "s/bsv-wasm/bsv-wasm-bundler/" ./pkg/package.json
	wasm-pack publish ./pkg


create-isomorphic:
	rollup pkg/node/bsv_wasm.js --format umd --name bsv_wasm_iso --file pkg/node/bsv_wasm_iso.js