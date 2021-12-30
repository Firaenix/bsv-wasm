check-format:
	cargo fmt -- --check && cargo clippy -- -Dwarnings

build-web:
	wasm-pack build --release --out-dir ./pkg/web --target web -- --features wasm-bindgen-exports

build-bundler:
	wasm-pack build --release --out-dir ./pkg/bundler --target bundler -- --features wasm-bindgen-exports

build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs -- --features wasm-bindgen-exports

build-deno:
	cargo build --target wasm32-unknown-unknown --features wasm-bindgen-exports
	wasm-bindgen ./target/wasm32-unknown-unknown/release/bsv_wasm.wasm --out-dir pkg/deno --target web --reference-types --weak-refs

build-wasm:
	make build-web ; make build-bundler ; make build-nodejs

test-node:
	make build-nodejs && pushd ./examples/node-test && yarn test ; popd

wasm-tests:
	wasm-pack test --node --  --features wasm-bindgen-exports

publish-node:
	# make sure not to call make build-* because wasm-pack doesnt allow you to specify subdirectories.
	wasm-pack build --release --target nodejs -- --features wasm-bindgen-exports
	wasm-pack publish ./pkg

publish-web:
	wasm-pack build --release --target web -- --features wasm-bindgen-exports
	sed -i "s/bsv-wasm/bsv-wasm-web/" ./pkg/package.json
	wasm-pack publish ./pkg

publish-bundler:
	wasm-pack build --release --target bundler -- --features wasm-bindgen-exports
	sed -i "s/bsv-wasm/bsv-wasm-bundler/" ./pkg/package.json
	wasm-pack publish ./pkg

create-isomorphic:
	rollup pkg/node/bsv_wasm.js --format umd --name bsv_wasm_iso --file pkg/node/bsv_wasm_iso.js