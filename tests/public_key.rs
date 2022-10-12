#[cfg(test)]
mod tests {
    use bsv::{PrivateKey, PublicKey};

    #[test]
    fn pub_key_from_private_key() {
        // Arrange
        let private_key = PrivateKey::from_hex("E9873D79C6D87DC0FB6A5778633389F4453213303DA61F20BD67FC233AA33262").unwrap();

        // Act
        let pub_key = PublicKey::from_private_key(&private_key);
        let pub_key_hex = pub_key.to_hex().unwrap();

        // Assert
        assert_eq!(pub_key_hex, "02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9")
    }

    #[test]
    fn pub_key_from_hex() {
        // Arrange
        let pub_key = PublicKey::from_hex("02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9").unwrap();

        // Act
        let pub_key_hex = pub_key.to_hex().unwrap();

        // Assert
        assert_eq!(pub_key_hex, "02588d202afcc1ee4ab5254c7847ec25b9a135bbda0f2bc69ee1a714749fd77dc9")
    }
}
