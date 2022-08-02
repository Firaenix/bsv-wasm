//use std::{
//fmt::Display,
//ops::{Neg, Not, Shl, Shr},
//};

use crate::{Script, ScriptBit, Transaction};
use errors::InterpreterError;
//use num_bigint::{BigInt, Sign};
use serde::{Deserialize, Serialize};
//use stack_trait::ScriptStack;

mod errors;
mod stack_trait;
pub mod state;
pub use state::*;
mod script_matching;

#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Status {
    Running,
    Finished,
}

impl Default for Status {
    fn default() -> Self {
        Status::Running
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TxScript {
    pub(crate) tx: Transaction,
    pub(crate) input_index: usize,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Interpreter {
    pub(crate) script_bits: Vec<ScriptBit>,
    pub(crate) script_index: usize,
    pub(crate) state: State,
    pub(crate) tx_script: Option<TxScript>,
}

impl Interpreter {
    pub fn from_transaction_and_script_bits(tx: Transaction, txin: usize, script_bits: Vec<ScriptBit>) -> Interpreter {
        Interpreter {
            script_bits,
            script_index: 0,
            state: State::default(),
            tx_script: Some(TxScript { tx, input_index: txin }),
        }
    }

    /// Runs the script to completion
    pub(crate) fn run_impl(&mut self) -> Result<(), InterpreterError> {
        while let Some(state) = self.next_impl()? {
            println!("=============NEXT==============");
            println!("{}", state);
        }

        Ok(())
    }

    pub(crate) fn next_impl(&mut self) -> Result<Option<State>, InterpreterError> {
        let script_bits = &self.script_bits.clone();
        let index = self.script_index;
        let new_state = match script_bits.get(index) {
            Some(v) => {
                // println!("Next Script Bit {:?}", v);

                Interpreter::match_script_bit(self, v)?
            }
            None => {
                self.state.status = Status::Finished;
                return Ok(None);
            }
        };

        self.script_index += 1;
        self.state = new_state;

        Ok(Some(self.state.clone()))
    }
}

/// Both WASM and Rust functionality
impl Interpreter {
    pub fn from_script(script: &Script) -> Interpreter {
        Interpreter {
            script_bits: script.to_script_bits(),
            script_index: 0,
            state: State::default(),
            tx_script: None,
        }
    }

    pub fn script(&self) -> Script {
        Script::from_script_bits(self.script_bits.clone())
    }
    /// Get the interpreter's script index.
    #[must_use]
    pub fn script_index(&self) -> usize {
        self.script_index
    }

    /// Get a reference to the interpreter's state.
    #[must_use]
    pub fn state(&self) -> State {
        self.state.clone()
    }

    /// Get a reference to the interpreter's tx script.
    #[must_use]
    pub fn tx_script(&self) -> Option<TxScript> {
        self.tx_script.clone()
    }
}

// /// WASM Only functionality
// #[cfg(all(target_arch = "wasm32"))]
// #[cfg_attr(all(target_arch = "wasm32"), wasm_bindgen)]
// impl Interpreter {
//     pub fn from_transaction(tx: &Transaction, txin: usize) -> Result<Interpreter, JsError> {
//         let script_bits = match tx
//                 .get_input(txin as usize)
//                 .unwrap()
//                 .get_finalised_script()? {
//                     Ok(v)
//                 }
//                 .to_script_bits();

//         Ok(Interpreter::from_transaction_impl(tx.clone(), txin, script_bits)?)
//     }

//     pub fn run(&mut self) -> Result<(), JsError> {
//         Ok(self.run_impl()?)
//     }

//     pub fn next(&mut self) -> Result<Option<State>, JsError> {
//         Ok(self.next_impl()?)
//     }
// }

/// Rust Only Functionality
impl Interpreter {
    pub fn from_transaction(tx: &Transaction, txin: usize) -> Result<Interpreter, InterpreterError> {
        let script_bits = tx.get_input(txin as usize).unwrap().get_finalised_script_impl()?.to_script_bits();
        Ok(Interpreter::from_transaction_and_script_bits(tx.clone(), txin, script_bits))
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        self.run_impl()
    }

    pub fn next(&mut self) -> Result<Option<State>, InterpreterError> {
        self.next_impl()
    }

    /// Get a reference to the interpreter's script bits.
    #[must_use]
    pub fn script_bits(&self) -> Vec<ScriptBit> {
        self.script_bits.clone()
    }
}

// Interpreter is an iterator, very useful incase we want to build a debugger
// impl FallibleStreamingIterator for Interpreter {
//     type Item = State;
//     type Error = ScriptError;

//     fn advance(&mut self) -> Result<(), Self::Error> {
//         let script_bits = &self.script_bits.clone();
//         let index = self.script_index;
//         let new_state = match script_bits.get(index) {
//             Some(v) => {
//                 // println!("Next Script Bit {:?}", v);

//                 Interpreter::match_script_bit(self, v)?
//             }
//             None => {
//                 self.state.status = Status::Finished;
//                 return Ok(());
//             }
//         };

//         self.script_index += 1;
//         self.state = new_state;

//         Ok(())
//     }

//     fn get(&self) -> Option<&Self::Item> {
//         match self.state.status {
//             Status::Running => Some(&self.state),
//             Status::Finished => None,
//         }
//     }
// }
