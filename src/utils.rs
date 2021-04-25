use serde::Serializer;

pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


pub fn to_hex<S>(vec: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
    where S: Serializer
{
  let hex = hex::encode(vec);

  serializer.serialize_str(&hex)
}