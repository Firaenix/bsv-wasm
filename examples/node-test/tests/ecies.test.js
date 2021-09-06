import { Ecies, PrivKey, PubKey, KeyPair } from 'bsv'
import { PrivateKey, PublicKey, ECIES, ECDH, ECIESCiphertext  } from '../../../pkg/node/bsv_wasm';
import { assert, util } from 'chai';

describe("ECIES Tests", function() {
  it('ECIES (Send to Self) Derived keys matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender + Recipient
    let bob = PrivateKey.fromRandom();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.toWIF()));

    let cipher_keys = ECIES.deriveCipherKeys(bob, bob.toPublicKey());

    let cipher_keys_js = Ecies.ivkEkM(bobJS.privKey, bobJS.pubKey);
    
    let iv = cipher_keys.get_iv();
    let ke = cipher_keys.get_ke();
    let km = cipher_keys.get_km();

    assert.equal(Buffer.from(iv).toString('hex'), cipher_keys_js.iv.toString('hex'), "IVs dont match");
    assert.equal(Buffer.from(ke).toString('hex'), cipher_keys_js.kE.toString('hex'), "KEs dont match");
    assert.equal(Buffer.from(km).toString('hex'), cipher_keys_js.kM.toString('hex'), "KMs dont match");
  });

  it('ECIES BIE (Ephemeral/Anonymous) Message matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Recipient
    let bob = PrivateKey.fromRandom();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.toWIF()));

    let ciphertext = ECIES.encryptWithEphemeralKey(message, bob.toPublicKey(), false);
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, null);
    
    // Cant compare ciphertexts because different ephemeral keys
    //  assert.equal(Buffer.from(ciphertext).toString('hex'), ciphertextJs.toString('hex'));

    let plaintext = ECIES.decrypt(ciphertext, bob, ciphertext.extractPublicKey());
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

    let ciphertext = ECIES.encrypt(message, bob, bob.toPublicKey(), false);
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, bobJS);
    
    assert.equal(Buffer.from(ciphertext.toBytes()).toString('hex'), ciphertextJs.toString('hex'), "Ciphertexts dont match");

    let plaintext = ECIES.decrypt(ciphertext, bob, bob.toPublicKey());
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

    let ciphertext = ECIES.encrypt(message, alice, bob.toPublicKey(), false);
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, aliceJS);

    assert.equal(Buffer.from(ciphertext.toBytes()).toString('hex'), ciphertextJs.toString('hex'), "Ciphertexts dont match");

    let alicePub = alice.toPublicKey();
    let plaintext = ECIES.decrypt(ciphertext, bob, alicePub);
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, null);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });

  it('ECIES (Send to Self) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender + Recipient
    let alice = PrivateKey.fromRandom();

    let alice_secret_stuff = alice.encryptMessage(message);
    let decrypted = alice.decryptMessage(alice_secret_stuff, alice.toPublicKey());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender
    let alice = PrivateKey.fromRandom();
    // Recipient
    let bob = PrivateKey.fromRandom();
    let bob_pub = bob.toPublicKey();

    // Alice does:
    let message_for_bob = bob_pub.encryptMessage(message, alice);

    // Bob does:
    let decrypted = bob.decryptMessage(message_for_bob, alice.toPublicKey());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party - Anonymous) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Recipient
    let bob = PrivateKey.fromRandom();
    let bob_pub = bob.toPublicKey();

    // Alice does:
    let message_for_bob = ECIES.encryptWithEphemeralKey(message, bob_pub);

    // Bob does:
    let decrypted = bob.decryptMessage(message_for_bob, message_for_bob.extractPublicKey());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party - Private) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender
    let alice = PrivateKey.fromRandom();
    // Recipient
    let bob = PrivateKey.fromRandom();
    let bob_pub = bob.toPublicKey();

    // Alice does:
    let message_for_bob = ECIES.encrypt(message, alice, bob_pub, true);

    // Bob does:
    let decrypted = bob.decryptMessage(message_for_bob, alice.toPublicKey());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party - Anonymous) Encode/Decode methods', () => {
let message = Buffer.from("Hello, Bitcoin.");

// Recipient
let bob = PrivateKey.fromRandom();
let bob_pub = bob.toPublicKey();

// Alice does:
let message_for_bob = ECIES.encryptWithEphemeralKey(message, bob_pub);
let message_bytes = message_for_bob.toBytes();

// Bob does:
let received_message = ECIESCiphertext.fromBytes(message_bytes, true);
let decrypted = bob.decryptMessage(received_message, message_for_bob.extractPublicKey());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });
});
