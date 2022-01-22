use secp256k1::hashes::sha256;
use secp256k1::rand::rngs::OsRng;
use secp256k1::{Message, Secp256k1};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn big_computation() {
    let secp = Secp256k1::new();
    let mut rng = OsRng::new().expect("OsRng");
    let (secret_key, public_key) = secp.generate_keypair(&mut rng);
    let message = Message::from_hashed_data::<sha256::Hash>("Hello World!".as_bytes());

    let sig = secp.sign_ecdsa(&message, &secret_key);

    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());

    alert(&format!("Big computation in Rust: {}", sig));
}

#[wasm_bindgen]
pub fn welcome(name: &str) {
    alert(&format!("Hello {}, from Rust!", name));
}
