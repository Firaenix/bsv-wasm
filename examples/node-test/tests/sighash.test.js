import {Transaction, SigHash, PrivateKey, Script} from '../../../pkg/bsv_wasm';
import { assert, util } from 'chai';
import { Tx, PrivKey, Script as JSScript, KeyPair, Sig, Bn } from "bsv";

describe("SigHash Tests", function() {
  it('SIGHASH_SINGLE Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_SINGLE | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.SINGLE | SigHash.FORKID, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_SINGLE | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.SINGLE | SigHash.FORKID, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_SINGLE (InputsOutput) Signed Tx matches BSV.JS ', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_SINGLE | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.InputsOutput, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_SINGLE | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.SINGLE | SigHash.FORKID, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_ALL Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_ALL | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.InputsOutputs, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_ALL | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.InputsOutputs, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_NONE Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_NONE | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.Inputs, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_NONE | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.Inputs, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });


  it('SIGHASH_NONE NO FORK_ID Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_NONE, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.NONE, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_NONE, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.NONE, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_ALL NO FORK_ID Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_ALL, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.ALL, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_ALL, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.ALL, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_SINGLE NO FORK_ID Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_SINGLE, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.SINGLE, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_SINGLE, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.SINGLE, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_SINGLE | ANYONECANPAY (InputOutput) Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_SINGLE | Sig.SIGHASH_ANYONECANPAY | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.InputOutput, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_SINGLE | Sig.SIGHASH_ANYONECANPAY | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.InputOutput, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_ALL | ANYONECANPAY (InputOutputs) Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_ALL | Sig.SIGHASH_ANYONECANPAY | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.InputOutputs, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_ALL | Sig.SIGHASH_ANYONECANPAY | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.InputOutputs, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });

  it('SIGHASH_NONE | ANYONECANPAY (Input) Signed Tx matches BSV.JS', () => {
    const wasm_private_key = PrivateKey.fromWIF("L31JUXCGspUREe9Gya8F2WWjeoRz3bb8AQzJjAP8ntGYp37oYdSx");
    const wasm_script = Script.fromASMString("OP_0 OP_RETURN");
    const wasm_tx = Transaction.fromHex("01000000029e8d016a7b0dc49a325922d05da1f916d1e4d4f0cb840c9727f3d22ce8d1363f000000008c493046022100e9318720bee5425378b4763b0427158b1051eec8b08442ce3fbfbf7b30202a44022100d4172239ebd701dae2fbaaccd9f038e7ca166707333427e3fb2a2865b19a7f27014104510c67f46d2cbb29476d1f0b794be4cb549ea59ab9cc1e731969a7bf5be95f7ad5e7f904e5ccf50a9dc1714df00fbeb794aa27aaff33260c1032d931a75c56f2ffffffffa3195e7a1ab665473ff717814f6881485dc8759bebe97e31c301ffe7933a656f020000008b48304502201c282f35f3e02a1f32d2089265ad4b561f07ea3c288169dedcf2f785e6065efa022100e8db18aadacb382eed13ee04708f00ba0a9c40e3b21cf91da8859d0f7d99e0c50141042b409e1ebbb43875be5edde9c452c82c01e3903d38fa4fd89f3887a52cb8aea9dc8aec7e2c9d5b3609c03eb16259a2537135a1bf0f9c5fbbcbdbaf83ba402442ffffffff02206b1000000000001976a91420bb5c3bfaef0231dc05190e7f1c8e22e098991e88acf0ca0100000000001976a9149e3e2d23973a04ec1b02be97c30ab9f2f27c3b2c88ac00000000");

    const js_private_key = PrivKey.fromWif(wasm_private_key.toWIF(true));
    const js_tx = Tx.fromHex(wasm_tx.toHex());
    const js_script = JSScript.fromAsmString(wasm_script.toASMString());
    
    assert.equal(js_tx.toHex(), wasm_tx.toHex());
    assert.equal(js_private_key.toWif(), wasm_private_key.toWIF(true));
    assert.equal(js_script.toAsmString(), wasm_script.toASMString());

    let js_sighash = js_tx.sighash(Sig.SIGHASH_NONE | Sig.SIGHASH_ANYONECANPAY | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sighash = wasm_tx.sighash(0, SigHash.Input, wasm_script, BigInt(0));

    assert.equal(Buffer.from(wasm_sighash).toString('hex'), js_sighash.toString('hex'), "Sighash functions do not match")

    let js_sig = js_tx.sign(KeyPair.fromPrivKey(js_private_key), Sig.SIGHASH_NONE | Sig.SIGHASH_ANYONECANPAY | Sig.SIGHASH_FORKID, 0, js_script, Bn(0));
    let wasm_sig = wasm_tx.sign(wasm_private_key, SigHash.Input, 0, wasm_script, BigInt(0));

    assert.equal(wasm_sig.toHex(), js_sig.toHex(), "Signed Sighash buffers dont match")
  });
});