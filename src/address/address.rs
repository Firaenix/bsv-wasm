use stdweb::js_export;

// use bitcoin_hashes::sha256;
// use secp256k1::{Message, Secp256k1, rand::rngs::OsRng};

#[js_export]

pub fn greet() -> Vec<u8> {
  // let secp = Secp256k1::new();
  // let mut rng = OsRng::new().expect("OsRng");
  // let (secret_key, public_key) = secp.generate_keypair(&mut rng);
  
  
  // let message = Message::from_hashed_data::<sha256::Hash>("Hello world!".as_bytes());

  // let sig = secp.sign(&message, &secret_key);
  // assert!(secp.verify(&message, &sig, &public_key).is_ok());

  // sig.serialize_compact().into()

  "Hello World!".into()
}