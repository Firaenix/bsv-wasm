// import { PrivateKey, PublicKey, Signature, Script, Transaction, TxOut } from "bsv-wasm";

// let key = PrivateKey.fromRandom();
// document.body.innerText = key.toWIF();


// let message = "Hello";
// let signature = key.signMessage(new TextEncoder().encode(message));
// document.body.innerText = document.body.innerText + '\n' + `Message: ${message}` + '\n' + `Signed text: ${signature.toHex()}`

// let pub_key = PublicKey.fromPrivateKey(key);
// document.body.innerText = document.body.innerText + '\n' + `Pub Key: ${pub_key.toHex()}`

// let verify_sig = Signature.fromHexDER(signature.toHex(), false);
// document.body.innerText = document.body.innerText + '\n' + `Verfied?: ${verify_sig.verifyMessage(new TextEncoder().encode(message), pub_key)}`

// let tx = new Transaction(1, 0);
// tx.addOutput(new TxOut(BigInt(400000000000), Script.fromASMString("OP_0 OP_RETURN")))
// tx.addOutput(new TxOut(BigInt(4000), Script.fromASMString("OP_0 OP_RETURN")))

// // let totalOutSats = tx.getOutput(0).getValue() + tx.getOutput(1).getValue();

// document.body.innerText = document.body.innerText + '\n' + `Tx ${tx} Tx Sats out?: ${tx.satoshisOut()} Type?: ${typeof(tx.satoshisOut())}`

import { Hash, TwetchPay, Wallet } from "twetch-sdk";
let result = await Wallet.from_seed_and_token("derive theory horse dash deny awkward interest human child hybrid awesome final", "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VyIjp7ImlkIjoiMTYifSwiaWF0IjoxNjU5MTcxNTAzfQ.tqa8r77wfFurYT3IC9F_h4cpsTXXmqsRl837RcoGQro");

document.body.innerText = document.body.innerText + '\n' + `Hash: ${result.account_public_key().to_hex()}`