use serde::{Deserialize, Deserializer, Serializer};
use std::str::FromStr;
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


pub fn to_hex<S>(vec: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
  let hex = hex::encode(vec);

  serializer.serialize_str(&hex)
}

pub fn from_hex<'de, D>(deserialiser: D) -> Result<Vec<u8>, D::Error> where D: Deserializer<'de> {
  let buf = String::deserialize(deserialiser)?;

  hex::decode(buf).map_err(serde::de::Error::custom)
}