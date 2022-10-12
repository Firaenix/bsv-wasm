#[cfg(test)]
mod private_key_tests {
    use bsv::keypair::*;

    #[test]
    fn import_private_key_and_verify() {
        let priv_key = "E9873D79C6D87DC0FB6A5778633389F4453213303DA61F20BD67FC233AA33262";

        let key = PrivateKey::from_hex(priv_key);

        assert_eq!(key.unwrap().to_hex(), priv_key.to_lowercase())
    }

    #[test]
    fn private_key_to_wif_verify() {
        let priv_key = "0C28FCA386C7A227600B2FE50B7CAE11EC86D3BF1FBE471BE89827E19D72AA1D";

        let key = PrivateKey::from_hex(priv_key).unwrap();

        let wif = key.compress_public_key(false).to_wif();
        assert_eq!(wif.unwrap(), "5HueCGU8rMjxEXxiPuD5BDku4MkFqeZyd4dZ1jvhTVqvbTLvyTJ");

        let wif2 = key.compress_public_key(true).to_wif();
        assert_eq!(wif2.unwrap(), "KwdMAjGmerYanjeui5SHS7JkmpZvVipYvB2LJGU1ZxJwYvP98617");
    }

    #[test]
    fn wif_to_private_key_uncompressed() {
        let wif = "5HueCGU8rMjxEXxiPuD5BDku4MkFqeZyd4dZ1jvhTVqvbTLvyTJ";

        let key = PrivateKey::from_wif(wif).unwrap();

        let private_key_hex = key.to_hex();

        assert_eq!(private_key_hex, "0C28FCA386C7A227600B2FE50B7CAE11EC86D3BF1FBE471BE89827E19D72AA1D".to_lowercase())
    }

    #[test]
    fn wif_to_private_key_compressed() {
        let wif = "L5EZftvrYaSudiozVRzTqLcHLNDoVn7H5HSfM9BAN6tMJX8oTWz6";

        let key = PrivateKey::from_wif(wif).unwrap();

        let private_key_hex = key.to_hex();

        assert_eq!(private_key_hex, "ef235aacf90d9f4aadd8c92e4b2562e1d9eb97f0df9ba3b508258739cb013db2".to_lowercase())
    }
}
