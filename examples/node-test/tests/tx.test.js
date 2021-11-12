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
    tx.addInput(txin_1);
    let txin_3 = new TxIn(
      Buffer.from(
        "4fe512f97769bc2fe47b0dadb1767404ebe2be50b3ea39a9b93d6325ee287e9a",
        "hex"
      ),
      2,
      Script.fromASMString("OP_0 OP_RETURN"),
      0
    );
    tx.addInput(txin_1);

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
});
