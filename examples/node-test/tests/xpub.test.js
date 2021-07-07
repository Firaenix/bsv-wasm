import { ExtendedPublicKey } from "../../../pkg/node/bsv_wasm";
import { Bip32 } from 'bsv';
import { assert } from "chai";



describe('XPub Tests', function () {
  it('XPubs match BSV.JS', () => {
    let xpub_wif = "xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu";

    let xpub_wasm = ExtendedPublicKey.fromString(xpub_wif);
    let xpub_js = Bip32.fromString(xpub_wif);

    assert.equal(xpub_wasm.toString(), xpub_js.toString());
  })

  it('XPub derivations match BSV.JS', () => {
    let xpub_wif = "xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu";
    let path = "m/0/0/0/0"

    let xpub_wasm = ExtendedPublicKey.fromString(xpub_wif);
    let xpub_js = Bip32.fromString(xpub_wif);

    assert.equal(xpub_wasm.derive(path).toString(), xpub_js.derive(path).toString());
  })

  it('XPub hardened derivations match BSV.JS - Throws Error', () => {
    let xpub_wif = "xpub67uA5wAUuv1ypp7rEY7jUZBZmwFSULFUArLBJrHr3amnymkUEYWzQJz13zLacZv33sSuxKVmerpZeFExapBNt8HpAqtTtWqDQRAgyqSKUHu";
    let path = "m/0'/0'/0'/0'"

    let xpub_wasm = ExtendedPublicKey.fromString(xpub_wif);
    let xpub_js = Bip32.fromString(xpub_wif);

    assert.throws(() => xpub_wasm.derive(path).toString());
    assert.throws(() => xpub_js.derive(path).toString());
  })

  it('XPub loop derivations match BSV.JS', () => {
    let xpub_wasm = ExtendedPublicKey.fromRandom();
    let xpub_wif = xpub_wasm.toString()
    let path = "m/0/0/0/0"

    let xpub_js = Bip32.fromString(xpub_wif);

    assert.equal(xpub_wasm.toString(), xpub_js.toString());
    assert.equal(xpub_wasm.derive(path).toString(), xpub_js.derive(path).toString());

    for (let index = 0; index < 100; index++) {
      const element = path + '/' + index;
      assert.equal(xpub_wasm.derive(element).toString(), xpub_js.derive(element).toString());
    }
  })
});
