pub trait ToHex {
    fn to_hex(&self) -> String;
}

impl ToHex for Vec<u8> {
    fn to_hex(&self) -> String {
        hex::encode(self)
    }
}

impl ToHex for [u8] {
    fn to_hex(&self) -> String {
        hex::encode(self)
    }
}
