# BSV.WASM

A Rust/WASM Library to interact with Bitcoin SV   

## Installation
NodeJS:  
`npm i bsv-wasm --save`

Web:  
`npm i bsv-wasm-web --save`

Rust:  
https://crates.io/crates/bsv-wasm

## Features
- Hash (SHA256, SHA256d, SHA1, RIPEMD160, Hash160, SHA512)
- ECDSA (Private Key, Public Key, Signatures)
- Transaction (Building, Serialising, Deserialising)
- Script (Serialising, Deserialising)
- Addresses (P2PKH)

## TODO:
- [x] SigHash Support
- [ ] ECIES
- [ ] Script Builder
- [ ] Isomorphic Package for JS
- [ ] Write documentation (Inline on functions and structs)
- [x] BIP32
- [x] BIP44
- [ ] Testnet Support
- [x] Nicer way to pass an array of TxIn and TxOut into things (currenly limited by https://github.com/rustwasm/wasm-bindgen/issues/111)

## Will not do:
- Mnemonics

### Thanks
- Brenton Gunning [(rust-sv)](https://github.com/brentongunning/rust-sv)
- Moneybutton Team [(bsv.js)](https://github.com/moneybutton/bsv)
- [Bitping Team](https://bitping.com)
- [learnmeabitcoin.com](https://learnmeabitcoin.com)
- [Bitcoin SV Wiki](https://wiki.bitcoinsv.io)