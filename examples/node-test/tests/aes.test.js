import { AES, AESAlgorithms } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { assert, util } from 'chai';
import { Aescbc } from "bsv";
import crypto from 'crypto';

describe("AES Tests", function() {
    it('AES256-CBC Encrypted Message matches BSV.JS', () => {
      const cipherKeyBuf = crypto.randomBytes(32)
      cipherKeyBuf.fill(0x10)
      const ivBuf = crypto.randomBytes(16)
      ivBuf.fill(0)
      const messageBuf = Buffer.from("Hello world");
      const encBuf = Aescbc.encrypt(messageBuf, cipherKeyBuf, ivBuf, false);

      let encrypted = AES.encrypt(cipherKeyBuf, ivBuf, messageBuf, AESAlgorithms.AES256_CBC);
      assert.equal(Buffer.from(encrypted).toString('hex'), encBuf.toString('hex'))
    });

    it('AES128-CBC Encrypted Message matches BSV.JS', () => {
      const cipherKeyBuf = crypto.randomBytes(16)
      cipherKeyBuf.fill(0x10)
      const ivBuf = crypto.randomBytes(16)
      
      ivBuf.fill(0)
      const messageBuf = Buffer.from("Hello world");
      const encBuf = Aescbc.encrypt(messageBuf, cipherKeyBuf, ivBuf, false);

      let encrypted = AES.encrypt(cipherKeyBuf, ivBuf, messageBuf, AESAlgorithms.AES128_CBC);
      assert.equal(Buffer.from(encrypted).toString('hex'), encBuf.toString('hex'))
    });
});
