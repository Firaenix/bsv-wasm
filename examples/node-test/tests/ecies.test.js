import { Ecies, PrivKey, PubKey, KeyPair } from 'bsv'
import { PrivateKey, PublicKey, ECIES, ECDH, ECIESCiphertext  } from '../../../packages/bsv-wasm/pkg/node/bsv_wasm';
import { assert, util } from 'chai';

describe("ECIES Tests", function() {
  it('ECIES (Send to Self) Derived keys matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender + Recipient
    let bob = PrivateKey.from_random();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.to_wif()));

    let cipher_keys = ECIES.derive_cipher_keys(bob, bob.to_public_key());

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
    let bob = PrivateKey.from_random();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.to_wif()));

    let ciphertext = ECIES.encrypt_with_ephemeral_private_key(message, bob.to_public_key(), false);
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, null);
    
    // Cant compare ciphertexts because different ephemeral keys
    //  assert.equal(Buffer.from(ciphertext).toString('hex'), ciphertextJs.toString('hex'));

    let plaintext = ECIES.decrypt(ciphertext, bob, ciphertext.extract_public_key());
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, null);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });

  it('ECIES BIE (Send to Self) Message matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender + Recipient
    let bob = PrivateKey.from_random();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.to_wif()));

    let ciphertext = ECIES.encrypt(message, bob, bob.to_public_key(), false);
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, bobJS);
    
    assert.equal(Buffer.from(ciphertext.to_bytes()).toString('hex'), ciphertextJs.toString('hex'), "Ciphertexts dont match");

    let plaintext = ECIES.decrypt(ciphertext, bob, bob.to_public_key());
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, null);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });

  it('ECIES BIE (Send to Other Party) Message matches BSV.JS', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender
    let alice = PrivateKey.from_random();
    let aliceJS = KeyPair.fromPrivKey(PrivKey.fromWif(alice.to_wif()));

    // Recipient
    let bob = PrivateKey.from_random();
    let bobJS = KeyPair.fromPrivKey(PrivKey.fromWif(bob.to_wif()));

    let ciphertext = ECIES.encrypt(message, alice, bob.to_public_key(), false);
    let ciphertextJs = Ecies.electrumEncrypt(message, bobJS.pubKey, aliceJS);

    assert.equal(Buffer.from(ciphertext.to_bytes()).toString('hex'), ciphertextJs.toString('hex'), "Ciphertexts dont match");

    let alicePub = alice.to_public_key();
    let plaintext = ECIES.decrypt(ciphertext, bob, alicePub);
    let plaintextJs = Ecies.electrumDecrypt(ciphertextJs, bobJS.privKey, null);

    assert.equal(plaintextJs.toString('hex'), message.toString('hex'));
    assert.equal(Buffer.from(plaintext).toString('hex'), plaintextJs.toString('hex'));

    assert.equal(Buffer.from(plaintext).toString('hex'), message.toString('hex'));
  });

  it('ECIES (Send to Self) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender + Recipient
    let alice = PrivateKey.from_random();

    let alice_secret_stuff = alice.encrypt_message(message);
    let decrypted = alice.decrypt_message(alice_secret_stuff, alice.to_public_key());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender
    let alice = PrivateKey.from_random();
    // Recipient
    let bob = PrivateKey.from_random();
    let bob_pub = bob.to_public_key();

    // Alice does:
    let message_for_bob = bob_pub.encrypt_message(message, alice);

    // Bob does:
    let decrypted = bob.decrypt_message(message_for_bob, alice.to_public_key());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party - Anonymous) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Recipient
    let bob = PrivateKey.from_random();
    let bob_pub = bob.to_public_key();

    // Alice does:
    let message_for_bob = ECIES.encrypt_with_ephemeral_private_key(message, bob_pub);

    // Bob does:
    let decrypted = bob.decrypt_message(message_for_bob, message_for_bob.extract_public_key());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party - Private) Convenience methods', () => {
    let message = Buffer.from("Hello, Bitcoin.");

    // Sender
    let alice = PrivateKey.from_random();
    // Recipient
    let bob = PrivateKey.from_random();
    let bob_pub = bob.to_public_key();

    // Alice does:
    let message_for_bob = ECIES.encrypt(message, alice, bob_pub, true);

    // Bob does:
    let decrypted = bob.decrypt_message(message_for_bob, alice.to_public_key());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });

  it('ECIES (Send to other party - Anonymous) Encode/Decode methods', () => {
let message = Buffer.from("Hello, Bitcoin.");

// Recipient
let bob = PrivateKey.from_random();
let bob_pub = bob.to_public_key();

// Alice does:
let message_for_bob = ECIES.encrypt_with_ephemeral_private_key(message, bob_pub);
let message_bytes = message_for_bob.to_bytes();

// Bob does:
let received_message = ECIESCiphertext.from_bytes(message_bytes, true);
let decrypted = bob.decrypt_message(received_message, message_for_bob.extract_public_key());

    assert.equal(Buffer.from(decrypted).toString('hex'), message.toString('hex'))
  });
});
