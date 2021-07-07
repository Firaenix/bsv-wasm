import { AES, AESAlgorithms } from '../../../pkg/node/bsv_wasm';
import { assert, util } from 'chai';
import { Aescbc } from "bsv";

describe("AES Tests", function() {
    it('AES Encrypted Message matches BSV.JS', () => {
        const cipherKeyBuf = Buffer.alloc(256 / 8)
        cipherKeyBuf.fill(0x10)
        const ivBuf = Buffer.alloc(128 / 8)
        ivBuf.fill(0)
        const messageBuf = Buffer.from("Hello world");
        const encBuf = Aescbc.encrypt(messageBuf, cipherKeyBuf, ivBuf, false);

        let encrypted = AES.encrypt(cipherKeyBuf, ivBuf, messageBuf, AESAlgorithms.AES256_CBC);
        assert.equal(Buffer.from(encrypted).toString('hex'), encBuf.toString('hex'))
      });
});
