# BSV.WASM

A Rust/WASM Library to interact with Bitcoin SV   

## Installation
NodeJS:  
`npm i bsv-wasm --save`

Web:  
`npm i bsv-wasm-web --save`

Webpack:  
`npm i bsv-wasm-bundler --save`

Rust:  
https://crates.io/crates/bsv-wasm


## Usage
### Note: Rust and JS/TS method names and structs are the same

- Derive private key from XPriv and log out P2PKH String
`ExtendedPrivateKey.fromWIF('LMyWif...').getPrivateKey().getPublicKey().toAddress().toString()`


## Caveats
- Bitcoin Signed Message (BSM) struct does not implement any base64 string parsing, please pass the raw Signature type or call `fromCompactBytes` on the Signature struct and pass it to the BSM.verify function.


## Features
- Hash (SHA256, SHA256d, SHA1, RIPEMD160, Hash160, SHA512)
- KDF (PBKDF2)
- Encryption (AES-CBC, AES-CTR)
- ECDSA (Private Key, Public Key, Signatures)
- Transaction (Building, Serialising, Deserialising)
- Script (Serialising, Deserialising)
- Addresses (P2PKH)
- Sighash Support
- Extended Private Keys and Child Derivation (BIP32, BIP42)

## TODO:
- [ ] ECIES
- [ ] Script Builder
- [ ] Isomorphic Package for JS
- [ ] Write documentation (Inline on functions and structs)
- [ ] Testnet Support

## Will not do:
- Mnemonics

### Thanks
- Brenton Gunning [(rust-sv)](https://github.com/brentongunning/rust-sv)
- Moneybutton Team [(bsv.js)](https://github.com/moneybutton/bsv)
- [Bitping Team](https://bitping.com)
- [learnmeabitcoin.com](https://learnmeabitcoin.com)
- [Bitcoin SV Wiki](https://wiki.bitcoinsv.io)