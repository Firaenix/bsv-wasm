import {PrivateKey, Hash as WASMHash} from '../../../pkg/node/bsv_wasm';
import { assert, util } from 'chai';
import {PrivKey, Ecdsa, KeyPair, Hash } from "bsv";

describe("Signature Tests", function() {
    it('Raw Signature Matches Signature from BSV.JS', () => {
      let wif = "L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC"
      let jswifPrivateKey = PrivKey.fromWif(wif);
      let jsKeyPair = KeyPair.fromPrivKey(jswifPrivateKey);

      let signatureMessage = Buffer.from("Hello, Bitcoin.");

      let signature = Ecdsa.sign(Buffer.from(WASMHash.sha256(signatureMessage).toBytes()), jsKeyPair);

      assert.equal("3044022032c4ac1fe69db038e55e5848ddf99865167a4f5172d5acf910c7ac5d66729cb8022049821966a892afc494777f6445d4757e5662b60d9acb1b2c810a5001892774ac", signature.toHex());

      let wasmPrivKey = PrivateKey.fromWIF(wif);
      let wasmSignature = wasmPrivKey.sign(signatureMessage)

      assert.equal("3044022032c4ac1fe69db038e55e5848ddf99865167a4f5172d5acf910c7ac5d66729cb8022049821966a892afc494777f6445d4757e5662b60d9acb1b2c810a5001892774ac", wasmSignature.toHex())

      assert.equal(signature.toHex(), wasmSignature.toHex())
    });


    it('Raw Signature Matches Signature from BSV.JS', () => {
      let wif = "L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC"
      let jswifPrivateKey = PrivKey.fromWif(wif);
      let jsKeyPair = KeyPair.fromPrivKey(jswifPrivateKey);

      let signatureMessage = Buffer.from("01000000c0379c566167eef7a8bc0a14d15f77f84e0b094b81637626d68eb2c40f3d253f00000000000000000000000000000000000000000000000000000000000000003f36d1e82cd2f327970c84cbf0d4e4d116f9a15dd02259329ac40d7b6a018d9e0000000002006a0000000000000000ffffffffc7732d98e887792b43e5dae92a159010d22e47d60ed48b88ba7b6c12a3c9e7560000000043000000", "hex");

      let signature = Ecdsa.sign(Buffer.from(WASMHash.sha256d(signatureMessage).toBytes()), jsKeyPair);

      assert.equal(signature.toHex(), "30440220210e4c7968daf9e023e0d813419a8f1be9fd49bd98446205a26a97b5b7b216cd02201d8fea7429c0b94238d94dc910e21c4f6a749bdc28057ba9f7462f214f425e68", "1.");

      let wasmPrivKey = PrivateKey.fromWIF(wif);
      let wasmSignature = wasmPrivKey.sign(WASMHash.sha256(signatureMessage).toBytes())

      assert.equal(wasmSignature.toHex(), "30440220210e4c7968daf9e023e0d813419a8f1be9fd49bd98446205a26a97b5b7b216cd02201d8fea7429c0b94238d94dc910e21c4f6a749bdc28057ba9f7462f214f425e68", "2.")

      assert.equal(signature.toHex(), wasmSignature.toHex(), "3.")
    });
});
