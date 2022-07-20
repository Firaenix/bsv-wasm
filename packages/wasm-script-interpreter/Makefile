build-nodejs:
	wasm-pack build --release --out-dir ./pkg/node --target nodejs # Generate package.json, etc.
	cargo build --target wasm32-unknown-unknown --release
	wasm-bindgen ./target/wasm32-unknown-unknown/release/wasm_script_interpreter.wasm --out-dir pkg/node --target nodejs --weak-refs
	wasm-opt -O4 --dce ./pkg/node/wasm_script_interpreter_bg.wasm -o ./pkg/node/wasm_script_interpreter_bg.wasm
