#[cfg_attr(not(target_arch = "wasm32"), allow(unused_imports))]
#[cfg(test)]
mod tests {
    use bsv::hash::Hash;
    use pbkdf2::{
        password_hash::{Ident, PasswordHasher, Salt, SaltString},
        Params, Pbkdf2,
    };

    #[test]
    fn sha1_hash_test() {
        let hash = Hash::sha_1("Hello, Bitcoin.".as_bytes());

        assert_eq!(hash.to_hex(), "f630136ccdc9e6e9555375e656ac544e21768650");
    }

    #[test]
    fn sha256_hash_test() {
        let hash = Hash::sha_256("Hello, Bitcoin.".as_bytes());

        assert_eq!(hash.to_hex(), "6d3903a73c0e59bd509694473ad4932cc5f7def58973009942d86e47d9db3810");
    }

    #[test]
    fn sha256d_hash_test() {
        let hash = Hash::sha_256d("Hello, Bitcoin.".as_bytes());

        assert_eq!(hash.to_hex(), "dd22c3760a8e683ae7eaa0635a3cdba785970e442fb7908ce1d897ea43f16b72");
    }

    #[test]
    fn ripemd160_hash_test() {
        let hash = Hash::ripemd_160("Hello, Bitcoin.".as_bytes());

        assert_eq!(hash.to_hex(), "3250c48cd5be0b3df6ef6bf600c8a4f1131ceb67");
    }

    #[test]
    fn hash160_hash_test() {
        let hash = Hash::hash_160("Hello, Bitcoin.".as_bytes());

        let sha256 = Hash::sha_256("Hello, Bitcoin.".as_bytes());
        let manual_hash160 = Hash::ripemd_160(&sha256.to_bytes());

        assert_eq!(hash.to_hex(), manual_hash160.to_hex());
        assert_eq!(hash.to_bytes(), manual_hash160.to_bytes());
    }

    #[test]
    fn sha512_hash_test() {
        let hash = Hash::sha_512("Hello, Bitcoin.".as_bytes());

        assert_eq!(
            hash.to_hex(),
            "d76d8ade1b94820eaf73369b79112d664456c0b2ed47189341d2306b946de5dd6dc40d473902e22b5b9eba28613aae6df58d6e66d58c8b6ba5cafe96bc0e1c29"
        );
    }
}
