

#[cfg(test)]
mod tests {
    use bsv_rs::keypair::*;


  #[test]
  fn import_private_key_and_verify() {
    let priv_key = "E9873D79C6D87DC0FB6A5778633389F4453213303DA61F20BD67FC233AA33262";

    let key = PrivateKey::from_hex(priv_key.into());

    assert_eq!(key.unwrap().to_hex(), priv_key.to_lowercase())
  }

  #[test]
  fn private_key_to_wif_verify() {
    let priv_key = "0C28FCA386C7A227600B2FE50B7CAE11EC86D3BF1FBE471BE89827E19D72AA1D";

    let key = PrivateKey::from_hex(priv_key.into());

    let wif = key.unwrap().to_wif();

    assert_eq!(wif, "5HueCGU8rMjxEXxiPuD5BDku4MkFqeZyd4dZ1jvhTVqvbTLvyTJ")
  }
}

