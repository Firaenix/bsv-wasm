import { PrivateKey, PublicKey, Signature } from "bsv-wasm";

document.body.innerText = "Loading";

const mykey = PrivateKey.fromRandom();
console.log("muh private key", mykey.toWIF(true))


document.body.innerText = mykey.toWIF(true);