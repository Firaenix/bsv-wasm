import { assert, util } from "chai";
import { Script, Transaction, TxIn } from "../../../pkg/node/bsv_wasm";

describe("Transaction Tests", function () {
  it("Get Outpoints", () => {
    let tx = new Transaction(1, 0);
    let txin_1 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      0,
      Script.fromASMString("OP_0 OP_RETURN"),
      0
    );
    tx.addInput(txin_1);
    let txin_2 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      1,
      Script.fromASMString("OP_0 OP_RETURN"),
      0
    );
    tx.addInput(txin_2);
    let txin_3 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      2,
      Script.fromASMString("OP_0 OP_RETURN"),
      0
    );
    tx.addInput(txin_3);

    let outpoints = tx.getOutpoints();

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

  it("Get CBOR Buffer - toCompactBytes", () => {
    let tx = new Transaction(1, 0);
    let txin_1 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      0,
      Script.fromASMString("OP_0 OP_RETURN"),
      0
    );
    txin_1.setSatoshis(60000n);
    tx.addInput(txin_1);
    let txin_2 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      1,
      Script.fromASMString("OP_0 OP_RETURN"),
      0
    );
    txin_2.setSatoshis(620000n);
    tx.addInput(txin_2);
    let txin_3 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      2,
      Script.fromASMString("OP_3 OP_EQUAL"),
      0
    );
    txin_3.setSatoshis(12n);
    txin_3.setUnlockingScript(Script.fromASMString("OP_1 OP_2 OP_ADD"))
    tx.addInput(txin_3);

    let CBORBuffer = tx.toCompactBytes();

    assert.deepEqual(Buffer.from(CBORBuffer).toString('hex'), "a46776657273696f6e0166696e7075747383a56a707265765f74785f696478403466653531326639373736396263326665343762306461646231373637343034656265326265353062336561333961396239336436333235656532383765396164766f7574006a7363726970745f73696782644f505f30694f505f52455455524e6873657175656e636500687361746f7368697319ea60a56a707265765f74785f696478403466653531326639373736396263326665343762306461646231373637343034656265326265353062336561333961396239336436333235656532383765396164766f7574016a7363726970745f73696782644f505f30694f505f52455455524e6873657175656e636500687361746f736869731a000975e0a66a707265765f74785f696478403466653531326639373736396263326665343762306461646231373637343034656265326265353062336561333961396239336436333235656532383765396164766f7574026a7363726970745f73696782644f505f33684f505f455155414c6873657175656e63650070756e6c6f636b696e675f73637269707483644f505f31644f505f32664f505f414444687361746f736869730c676f757470757473806a6e5f6c6f636b74696d6500");

    let reconstructed_tx = Transaction.fromCompactBytes(CBORBuffer);

    assert.equal(reconstructed_tx.getInput(0).getSatoshis(), 60000n);

    let reconstructed_txin_3 = reconstructed_tx.getInput(2);

    assert.equal(reconstructed_txin_3.getSatoshis(), 12n);
    assert.equal(reconstructed_txin_3.getScriptSig().toASMString(),"OP_3 OP_EQUAL");
    assert.equal(Script.fromBytes(reconstructed_txin_3.getUnlockingScriptBytes()).toASMString(), "OP_1 OP_2 OP_ADD");
  });
});
