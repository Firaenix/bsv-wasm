import { ExtendedPrivateKey } from "../../../packages/bsv-wasm/pkg/node/bsv_wasm";
import { Bip32, Bip39 } from 'bsv';
import { assert } from "chai";
import crypto from 'crypto';



describe('XPriv Tests', function () {
  it('XPrivs match BSV.JS', () => {
    let xpriv_wif = "xprv9s21ZrQH143K2rdSf96bvxvYtHYjf2899A7M7S3Ka2jASLK6P3hs7Bg9snGVsArqAA2awhc26e5kqKDquKSkpZ6hXymjpCcUj1tRi17L4Bg";

    let xpriv_wasm = ExtendedPrivateKey.from_string(xpriv_wif);
    let xpriv_js = Bip32.fromString(xpriv_wif);

    assert.equal(xpriv_wasm.to_string(), xpriv_js.toString());
  })

  it('Short seed match BSV.JS', () => {
    let small_bytes = crypto.randomBytes(32);
    let xpriv_wasm = ExtendedPrivateKey.from_seed(small_bytes);
    let xpriv_js = Bip32.fromSeed(small_bytes);

    assert.equal(xpriv_wasm.to_string(), xpriv_js.toString());
  })

  it('Long seed match BSV.JS', () => {
    let large_bytes = crypto.randomBytes(64);
    let xpriv_wasm = ExtendedPrivateKey.from_seed(large_bytes);
    let xpriv_js = Bip32.fromSeed(large_bytes);

    assert.equal(xpriv_wasm.to_string(), xpriv_js.toString());
  })

  it('Mnemonic -> XPriv derivations match BSV.JS', () => {
    let mnemonic = 'vapor cabbage jacket unveil permit web live pyramid husband final plug metal'

    let xpriv_wasm = ExtendedPrivateKey.from_mnemonic(Buffer.from(mnemonic, 'utf8')).to_string();
    let xpriv_js = Bip32.fromSeed(new Bip39().fromString(mnemonic).toSeed());

    assert.equal(xpriv_wasm, xpriv_js.toString());
    assert.equal(xpriv_wasm, "xprv9s21ZrQH143K3kV5ByEVyeoaC6TbWS9T3UrQamHwMgpbTghuLXUfiSgeK1TRr1K9xWVcJKdtQawEM1RGwAfCzwPHJXSCEzTSze7ZnduyQaU");
  })

  it('XPriv derivations match BSV.JS', () => {
    let xpriv_wif = "xprv9s21ZrQH143K2rdSf96bvxvYtHYjf2899A7M7S3Ka2jASLK6P3hs7Bg9snGVsArqAA2awhc26e5kqKDquKSkpZ6hXymjpCcUj1tRi17L4Bg";
    let path = "m/0/0/0/0"

    let xpriv_wasm = ExtendedPrivateKey.from_string(xpriv_wif);
    let xpriv_js = Bip32.fromString(xpriv_wif);

    assert.equal(xpriv_wasm.derive_from_path(path).to_string(), xpriv_js.derive(path).toString());
  })

  it('XPriv hardened derivations match BSV.JS', () => {
    let xpriv_wif = "xprv9s21ZrQH143K2rdSf96bvxvYtHYjf2899A7M7S3Ka2jASLK6P3hs7Bg9snGVsArqAA2awhc26e5kqKDquKSkpZ6hXymjpCcUj1tRi17L4Bg";
    let path = "m/0'/0'/0'/0'"

    let xpriv_wasm = ExtendedPrivateKey.from_string(xpriv_wif);
    let xpriv_js = Bip32.fromString(xpriv_wif);

    assert.equal(xpriv_wasm.derive_from_path(path).to_string(), xpriv_js.derive(path).toString());
  })

  it('XPriv loop hardened derivations match BSV.JS', () => {
    let xpriv_wasm = ExtendedPrivateKey.from_random();
    let xpriv_wif = xpriv_wasm.to_string()
    let path = "m/0'/0'/0'/0'"

    let xpriv_js = Bip32.fromString(xpriv_wif);

    assert.equal(xpriv_wasm.to_string(), xpriv_js.toString());
    assert.equal(xpriv_wasm.derive_from_path(path).to_string(), xpriv_js.derive(path).toString());

    for (let index = 0; index < 100; index++) {
      const element = path + '/' + index;
      assert.equal(xpriv_wasm.derive_from_path(element).to_string(), xpriv_js.derive(element).toString());
    }
  })
});
