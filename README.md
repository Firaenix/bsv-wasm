# BSV.WASM/BSV-RS

A Rust/WASM Library to interact with Bitcoin SV   

## Installation
NodeJS 14.6+:  
`npm i bsv-wasm --save`

Web:  
`npm i bsv-wasm-web --save`

Webpack:  
`npm i bsv-wasm-bundler --save`

Rust:  
https://crates.io/crates/bsv

Deno:  
```js
import init, { Transaction } from "https://deno.land/x/bsv_wasm@{VERSION}/bsv_wasm.js"
await init();
```


## Usage
### Note: Rust and JS/TS method names and structs are the same

- Eg. Derive private key from XPriv and log out P2PKH String
`ExtendedPrivateKey.fromWIF('LMyWif...').toPrivateKey().toPublicKey().toAddress().toString()`

## Features
- Hash (SHA256, SHA256d, SHA1, RIPEMD160, Hash160, SHA512)
- KDF (PBKDF2)
- Encryption (AES-CBC, AES-CTR)
- ECDSA (Private Key, Public Key, Signatures)
- Transaction (Building, Serialising, Deserialising)
- Script (Serialising, Deserialising)
- Script Matching (ScriptTemplate)
- Addresses (P2PKH)
- Sighash Support
- Extended Private Keys and Child Derivation (BIP32, BIP42)
- Testnet support

## TODO:
- [ ] Split WASM modules from Rust - Release standalone bsv-rs
- [ ] Break WASM packages up to be more modular
- [ ] Allow Interop between WASM Packages
- [ ] Write documentation (Inline on functions and structs)

### Thanks
- Brenton Gunning [(rust-sv)](https://github.com/brentongunning/rust-sv)
- Moneybutton Team [(bsv.js)](https://github.com/moneybutton/bsv)
- [Bitping Team](https://bitping.com)
- [learnmeabitcoin.com](https://learnmeabitcoin.com)
- [Bitcoin SV Wiki](https://wiki.bitcoinsv.io)
