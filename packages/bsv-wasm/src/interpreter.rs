use wasm_bindgen::{prelude::*, JsError};
use crate::{ script::Script, transaction::Transaction };
use bsv::{ Interpreter as BSVInterpreter, State };

#[wasm_bindgen]
pub struct Interpreter(pub(crate) BSVInterpreter);


#[wasm_bindgen]
impl Interpreter {
    pub fn from_transaction(tx: Transaction, txin_idx: usize) -> Result<Interpreter, JsError>  {
        Ok(Interpreter(BSVInterpreter::from_transaction(&tx.0, txin_idx)?))
    }

    pub fn from_script(script: Script) -> Interpreter {
        Interpreter(BSVInterpreter::from_script(&script.0))
    }

    pub fn run(&mut self) -> Result<(), JsError> {
        Ok(self.0.run()?)
    }

    pub fn next(&mut self) -> Result<Option<JsState>, JsError> {
        let state = self.0.next()?;

        let js_state = state.map(|s| JsState(s));
        Ok(js_state)
    }

    pub fn get_state(&self) -> JsState {
        JsState(self.0.state())
    }
}


#[wasm_bindgen(js_name = State)]
pub struct JsState(pub(crate) State);

#[wasm_bindgen(js_name = Status)]
pub enum JsStatus {
 Running,
 Finished
}

impl From<bsv::Status> for JsStatus {
    fn from(s: bsv::Status) -> Self {
        match s {
            bsv::Status::Running => JsStatus::Running,
            bsv::Status::Finished => JsStatus::Finished,
        }
    }
}

#[wasm_bindgen]
impl JsState {
    pub fn get_executed_script(&self) -> Result<Script, JsError> {
        let asm_string: String = self.0.executed_opcodes.iter().map(|x| x.to_string()).fold(String::new(), |acc, x| format!("{} {}", acc, x));
        Ok(Script::from_asm_string(&asm_string)?)
    }

    pub fn get_stack(&self) -> Result<JsValue, JsError> {
        let stack = self.0.stack();

        Ok(serde_wasm_bindgen::to_value(stack)?)
    }

     pub fn get_alt_stack(&self) -> Result<JsValue, JsError> {
        let stack = &self.0.alt_stack;

        Ok(serde_wasm_bindgen::to_value(&stack)?)
    }

     pub fn get_status(&self) -> JsStatus {
         self.0.status.clone().into()
     }
}
