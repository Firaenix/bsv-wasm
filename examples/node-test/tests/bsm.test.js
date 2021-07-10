import { Bsm, PrivKey, PubKey, KeyPair, Address } from 'bsv'
import { PrivateKey, PublicKey, P2PKHAddress, SigningHash } from '../../../pkg/node/bsv_wasm';

import { assert, util } from 'chai';

describe("Bitcoin Signed Messages", function() {
    it('signed message matches BSV.JS', () => {
        let wif = "L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC"


        const priv_wasm = PrivateKey.fromWIF(wif);
        const pub_wasm = PublicKey.fromPrivateKey(priv_wasm)
        const address_wasm = P2PKHAddress.fromPubKey(pub_wasm)

        const priv_js = PrivKey.fromWif(wif);
        const pub_js = new PubKey().fromPrivKey(priv_js);
        const address_js = new Address().fromPubKey(pub_js);


        const message = Buffer.from('Hello, Bitcoin', 'utf8');

        const signature_js = Bsm.sign(message, new KeyPair().fromPrivKey(priv_js))
        const verification_js = Bsm.verify(message, signature_js, address_js)

        const buf = Buffer.from('18426974636f696e205369676e6564204d6573736167653a0a0e48656c6c6f2c20426974636f696e', 'hex') // calculated by logging the buf inside Bsv.magicHash(message) before sha256d
        const signature_wasm = Buffer.from(priv_wasm.sign(buf, SigningHash.Sha256d, false).toCompactBytes()).toString('base64')
        const verification_wasm = Bsm.verify(message, signature_wasm, address_js)

        const validSignature = 'IEASldKxt6sTOO1vMc3x2wN2qa5iZAUUHcj+fzekoLpOL5fl/W8ZApmSGzT211K83hHD3EQ6VE4RFEezVmPWd6Q='
        assert.equal(address_js.toString(), address_wasm.toString());

        assert.equal(verification_js, true);
        assert.equal(validSignature, signature_js)

        assert.equal(verification_wasm, true);
        assert.equal(validSignature, signature_wasm)

        assert.equal(signature_wasm, signature_js)
      });
});
