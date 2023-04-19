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
#[derive(Default)]
pub enum Status {
    #[default]
    Running,
    Finished,
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
        while let Some(state) = self.next_impl() {
            let state = state?;
            println!("=============NEXT==============");
            println!("{}", state);
        }

        Ok(())
    }

    pub(crate) fn next_impl(&mut self) -> Option<Result<State, InterpreterError>> {
        let script_bits = &self.script_bits.clone();
        let index = self.script_index;
        let new_state = match script_bits.get(index) {
            Some(v) => match Interpreter::match_script_bit(self, v) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            },
            None => {
                self.state.status = Status::Finished;
                return None;
            }
        };

        self.script_index += 1;
        self.state = new_state;

        Some(Ok(self.state.clone()))
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

    pub fn from_transaction(tx: &Transaction, txin: usize) -> Result<Interpreter, InterpreterError> {
        let script_bits = tx.get_input(txin).unwrap().get_finalised_script_impl()?.to_script_bits();
        Ok(Interpreter::from_transaction_and_script_bits(tx.clone(), txin, script_bits))
    }

    pub fn run(&mut self) -> Result<(), InterpreterError> {
        self.run_impl()
    }

    /// Get a reference to the interpreter's script bits.
    #[must_use]
    pub fn script_bits(&self) -> Vec<ScriptBit> {
        self.script_bits.clone()
    }
}

impl Iterator for Interpreter {
    type Item = Result<State, InterpreterError>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_impl()
    }
}
