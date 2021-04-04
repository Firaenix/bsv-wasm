#[wasm_bindgen]
#[derive(Debug)]
pub struct Transaction {
  pub version: u32,
  pub inputs: Vec<TxIn>,
  pub outputs: Vec<TxOut>,
  pub nLockTime: u32;
}

#[wasm_bindgen]
impl Transaction {

}