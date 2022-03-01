import { ChainParams, P2PKHAddress } from '../../../pkg/node/bsv_wasm';
import { assert, util } from 'chai';

describe("P2PKH Address Tests", function() {
  it('Decodes a testnet address', () => {
    const address = P2PKHAddress.fromString("moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC")
    assert.equal(address.toString(), 'moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC')
  });

  it('Transforms a testnet address to a mainnet address', () => {
    const address = P2PKHAddress.fromString("moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC").setChainParams(new ChainParams());
    assert.equal(address.toString(), '18irYdwarX2sxFbcYXG47B1iMnM5rWxsem')
  });

  it('Transforms a mainnet address to a testnet address', () => {
    const address = P2PKHAddress.fromString("18irYdwarX2sxFbcYXG47B1iMnM5rWxsem").setChainParams(ChainParams.Testnet());
    assert.equal(address.toString(), 'moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC')
  });
});
