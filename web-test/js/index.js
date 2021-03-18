import {Hash} from "bsv";
import greenlet from 'greenlet';


(async () => {
  const {hash} = await import('../../pkg');

  let target = "00";

  console.log("Running WASM")
  
  let getWASMPOW = async () => {
    let nonce = 0;
    while (true) {
      let hashed = hash(new Uint8Array([nonce]));
  
      if (hashed.startsWith(target)) {
        return hashed;
      }
      nonce++;
    }
  };

  const start = Date.now();
  console.log(await getWASMPOW());
  const end = Date.now();

  let wasmTag = document.createElement("p");
  wasmTag.innerHTML = (`Total time to mine ${target} in WASM: ${end - start}ms`);;
  document.body.appendChild(wasmTag);

  console.log("Running BSVJS")
  

  let getJSPOW = async() => {
    let nonce = 0;
    
    while (true) {
      let hashed = Hash.sha256(Buffer.from([nonce]));
  
      if (hashed.toString('hex').startsWith(target)) {
        return hashed;
      }

      nonce++;
    }
  }

  const start2 = Date.now();
  console.log(await getJSPOW());
  const end2 = Date.now();

  let bsvTag = document.createElement("p");
  bsvTag.innerHTML = (`Total time to mine ${target} in BSV2: ${end2 - start2}ms`);
  document.body.appendChild(bsvTag);
})()