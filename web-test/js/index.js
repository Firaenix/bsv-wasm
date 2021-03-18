import {PrivKey} from "bsv";
(async () => {
  const {PrivateKey, PublicKey, P2PKHAddress} = await import('../../pkg/bsv_rs.js');

  console.log("Running WASM")
  const start = Date.now();
  let count = 0;
  while (count < 1000000) {
    PrivateKey.fromRandom();
    count++;
  }

  const end = Date.now();

  console.log("Total time to mine 1mil private keys in WASM:", end - start)


  console.log("Running BSVJS")
  const start2 = Date.now();
  let count2 = 0;
  while (count2 < 1000000) {
    PrivKey.fromRandom();
    count2++;
  }

  const end2 = Date.now();

  console.log("Total time to mine 1mil private keys in BSV2 (JS):", end2 - start2)
})()