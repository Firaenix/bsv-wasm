import { ChainParams, P2PKHAddress } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { assert, util } from 'chai';

describe("P2PKH Address Tests", function() {
  it('Decodes a testnet address', () => {
    const address = P2PKHAddress.from_string("moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC")
    assert.equal(address.to_address_string(), 'moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC')
  });

  it('Transforms a testnet address to a mainnet address', () => {
    const address = P2PKHAddress.from_string("moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC").set_chain_params(new ChainParams());
    assert.equal(address.to_address_string(), '18irYdwarX2sxFbcYXG47B1iMnM5rWxsem')
  });

  it('Transforms a mainnet address to a testnet address', () => {
    const address = P2PKHAddress.from_string("18irYdwarX2sxFbcYXG47B1iMnM5rWxsem").set_chain_params(ChainParams.testnet());
    assert.equal(address.to_address_string(), 'moEoqh2ZfYU8jN5EG6ERw6E3DmwnkuTdBC')
  });
});
