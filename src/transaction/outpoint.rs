use serde::{Serialize, Deserialize};


#[derive(Eq, PartialEq, Serialize, Deserialize)]
// Outpoint - A u8 slice of exactly 36 bytes in length
pub struct Outpoint(pub [u8;36]);
