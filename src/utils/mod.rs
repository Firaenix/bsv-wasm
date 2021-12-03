pub mod bytes;
pub use bytes::*;

use serde::{Deserialize, Deserializer, Serializer};

pub fn to_hex<S>(vec: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let hex = hex::encode(vec);

    serializer.serialize_str(&hex)
}

pub fn from_hex<'de, D>(deserialiser: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserialiser)?;

    hex::decode(buf).map_err(serde::de::Error::custom)
}

pub fn to_base58<S>(vec: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let base58 = bs58::encode(vec);

    serializer.serialize_str(&base58.into_string())
}

pub fn from_base58<'de, D>(deserialiser: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserialiser)?;

    bs58::decode(buf).into_vec().map_err(serde::de::Error::custom)
}
