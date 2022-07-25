import { assert, util } from "chai";
import { Script, Transaction, TxIn } from "../../../packages/bsv-wasm/pkg/node/bsv_wasm";

describe("Transaction Tests", function () {
  it("Get Outpoints", () => {
    let tx = new Transaction(1, 0);
    let txin_1 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      0,
      Script.from_asm_string("OP_0 OP_RETURN"),
      0
    );
    tx.add_input(txin_1);
    let txin_2 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      1,
      Script.from_asm_string("OP_0 OP_RETURN"),
      0
    );
    tx.add_input(txin_2);
    let txin_3 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      2,
      Script.from_asm_string("OP_0 OP_RETURN"),
      0
    );
    tx.add_input(txin_3);

    let outpoints = tx.get_outpoints();

    assert.deepEqual(Uint8Array.from(outpoints[0]), Uint8Array.from(Buffer.from(
      "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f00000000", 'hex'
    )));
    assert.deepEqual(Uint8Array.from(outpoints[1]), Uint8Array.from(Buffer.from(
      "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f01000000", 'hex'
    )));
    assert.deepEqual(Uint8Array.from(outpoints[2]), Uint8Array.from(Buffer.from(
      "9a7e28ee25633db9a939eab350bee2eb047476b1ad0d7be42fbc6977f912e54f02000000", 'hex'
    )));
  });

  it("Deserialise Coinbase hex", () => {
    let tx = Transaction.from_hex("01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff63038d361604747a77610840000000230000004e2f686f77206c6f6e672063616e207468697320626520746573742074657374206170706172656e746c7920707265747479206c6f6e67206f6b20776f772031323334353637383930313220f09fa68d2f0000000001c817a804000000001976a91454b34b1ba228ba1d75dca5a40a114dc0f13a268788ac00000000");
    assert.isTrue(tx.is_coinbase());
    assert.equal(tx.to_hex(), "01000000010000000000000000000000000000000000000000000000000000000000000000ffffffff63038d361604747a77610840000000230000004e2f686f77206c6f6e672063616e207468697320626520746573742074657374206170706172656e746c7920707265747479206c6f6e67206f6b20776f772031323334353637383930313220f09fa68d2f0000000001c817a804000000001976a91454b34b1ba228ba1d75dca5a40a114dc0f13a268788ac00000000");
  })

  it("Get CBOR Buffer - to_compact_bytes", () => {
    let tx = new Transaction(1, 0);
    let txin_1 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      0,
      Script.from_asm_string("OP_0 OP_RETURN"),
      0
    );
    txin_1.set_satoshis(60000n);
    tx.add_input(txin_1);


    let txin_2 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      1,
      Script.from_asm_string("OP_0 OP_RETURN"),
      0
    );
    txin_2.set_satoshis(620000n);
    tx.add_input(txin_2);


    let txin_3 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      2,
      Script.from_asm_string("OP_1 OP_2 OP_ADD"),
      0
    );
    txin_3.set_satoshis(12n);
    txin_3.set_locking_script(Script.from_asm_string("OP_3 OP_EQUAL"))
    tx.add_input(txin_3);

    let CBORBuffer = tx.to_compact_bytes();

    assert.deepEqual(Buffer.from(CBORBuffer).toString('hex'), "a46776657273696f6e0166696e7075747383a56a707265765f74785f696478403961376532386565323536333364623961393339656162333530626565326562303437343736623161643064376265343266626336393737663931326535346664766f75740070756e6c6f636b696e675f73637269707482644f505f30694f505f52455455524e6873657175656e636500687361746f7368697319ea60a56a707265765f74785f696478403961376532386565323536333364623961393339656162333530626565326562303437343736623161643064376265343266626336393737663931326535346664766f75740170756e6c6f636b696e675f73637269707482644f505f30694f505f52455455524e6873657175656e636500687361746f736869731a000975e0a66a707265765f74785f696478403961376532386565323536333364623961393339656162333530626565326562303437343736623161643064376265343266626336393737663931326535346664766f75740270756e6c6f636b696e675f73637269707483644f505f31644f505f32664f505f4144446873657175656e6365006e6c6f636b696e675f73637269707482644f505f33684f505f455155414c687361746f736869730c676f757470757473806a6e5f6c6f636b74696d6500");

    let reconstructed_tx = Transaction.from_compact_bytes(CBORBuffer);

    assert.equal(reconstructed_tx.get_input(0).get_satoshis(), 60000n);

    let reconstructed_txin_3 = reconstructed_tx.get_input(2);

    assert.equal(reconstructed_txin_3.get_satoshis(), 12n);
    assert.equal(reconstructed_txin_3.get_locking_script().to_asm_string(),"OP_3 OP_EQUAL");
    assert.equal(reconstructed_txin_3.get_unlocking_script().to_asm_string(), "OP_1 OP_2 OP_ADD");
  });
});
