


#[cfg(test)]
mod tests {
    // use wasm_bindgen::__rt::assert_not_null;

  #[test]
  fn pass() {
      let string = bsv_rs::sig();

      println!("{:#?}", string);
  }
}

