import { Ecies, PrivKey, PubKey, KeyPair } from 'bsv'
import { PrivateKey, PublicKey, ECIES  } from '../../../pkg/node/bsv_wasm';
import { assert, util } from 'chai';

describe("ECIES Tests", function() {
  it('ECIES BIE (Ephemeral/Anonymous) Message matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Recipient
    let bob = PrivateKey.fromRandom();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.toWIF()));

    let ciphertext = ECIES.encrypt(message, undefined, bob.getPublicKey());
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, null);
    
    // Cant compare ciphertexts because different ephemeral keys
    //  assert.equal(Buffer.from(ciphertext).toString('hex'), ciphertextJs.toString('hex'));

    let plaintext = ECIES.decrypt(ciphertext, bob, undefined);
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, null);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });

  it('ECIES BIE (Send to Self) Message matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender + Recipient
    let bob = PrivateKey.fromRandom();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.toWIF()));

    let ciphertext = ECIES.encrypt(message, bob, bob.getPublicKey());
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, bobJS);
    
    assert.equal(Buffer.from(ciphertext).toString('hex'), ciphertextJs.toString('hex'), "Ciphertexts dont match");

    let plaintext = ECIES.decrypt(ciphertext, bob, undefined);
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, null);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });

  it('ECIES BIE (Send to Other Party) Message matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender
    let alice = PrivateKey.fromRandom();
    let aliceJS = KeyPair.fromPrivKey(PrivKey.fromWif(alice.toWIF()));

    // Recipient
    let bob = PrivateKey.fromRandom();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.toWIF()));

    let ciphertext = ECIES.encrypt(message, alice, bob.getPublicKey());
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, aliceJS);

    assert.equal(Buffer.from(ciphertext).toString('hex'), ciphertextJs.toString('hex'), "Ciphertexts dont match");

    let plaintext = ECIES.decrypt(ciphertext, bob, alice.getPublicKey());
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, aliceJS.pubKey);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });
});
