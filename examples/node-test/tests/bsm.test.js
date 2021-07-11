import { Bsm, PrivKey, PubKey, KeyPair, Address } from 'bsv'
import { PrivateKey, PublicKey, P2PKHAddress, SigningHash, BSM,  } from '../../../pkg/node/bsv_wasm';

import { assert, util } from 'chai';

describe("Bitcoin Signed Messages", function() {
    it('signed message matches BSV.JS', () => {
        let wif = "L17y3TE8AgM6fiWFP4HsbaLnvuBJsQcFKYRoJoZULpTzeTCr2nEC"


        const priv_wasm = PrivateKey.fromWIF(wif);
        const pub_wasm = PublicKey.fromPrivateKey(priv_wasm)
        const address_wasm = P2PKHAddress.fromPubKey(pub_wasm)

        console.log("(Priv) Uncompressed Address:", priv_wasm.compressPublicKey(false).getPublicKey().toAddress().toString());
        console.log("(Priv) Compressed Address:", priv_wasm.compressPublicKey(true).getPublicKey().toAddress().toString());

        console.log("(Pub) Uncompressed Address:", pub_wasm.toDecompressed().toAddress().toString());
        console.log("(Pub) Compressed Address:", pub_wasm.toCompressed().toAddress().toString());

        console.log("(Addr) Address:", address_wasm.toString());

        const priv_js = PrivKey.fromWif(wif);
        const pub_js = new PubKey().fromPrivKey(priv_js);
        const address_js = new Address().fromPubKey(pub_js);


        const message = Buffer.from('Hello, Bitcoin', 'utf8');

        const signature_js = Bsm.sign(message, new KeyPair().fromPrivKey(priv_js))
        assert.equal(Bsm.verify(message, signature_js, address_js), true);

        // const buf = Buffer.from('18426974636f696e205369676e6564204d6573736167653a0a0e48656c6c6f2c20426974636f696e', 'hex') // calculated by logging the buf inside Bsv.magicHash(message) before sha256d
        
        let signature_wasm = BSM.signMessage(priv_wasm, message);

        console.log("WASM Sig Hex\n", Buffer.from(signature_wasm.toCompactBytes()).toString('hex'))
        console.log("JS Sig Hex\n", Buffer.from(signature_js, 'base64').toString('hex'))

        const signature_wasm_b64 = Buffer.from(signature_wasm.toCompactBytes()).toString('base64')
        const verification_wasm = Bsm.verify(message, signature_wasm_b64, address_js)
        assert.equal(verification_wasm, true);
      
        assert.equal(address_wasm.verifyBitcoinMessage(message, signature_wasm), true);
        assert.equal(BSM.verifyMessage(message, signature_wasm, address_wasm), true);
        assert.equal(BSM.isValidMessage(message, signature_wasm, address_wasm), true);
        
        const validSignature = 'IEASldKxt6sTOO1vMc3x2wN2qa5iZAUUHcj+fzekoLpOL5fl/W8ZApmSGzT211K83hHD3EQ6VE4RFEezVmPWd6Q='
        assert.equal(address_js.toString(), address_wasm.toString());

        
        assert.equal(validSignature, signature_js)

        assert.equal(validSignature, signature_wasm_b64)

        assert.equal(signature_wasm, signature_js)
      });
});
