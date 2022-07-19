use std::fmt::Display;

use crate::{OpCodes, ToHex};
use serde::{Serialize, Deserialize};

use crate::{Status};

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct State {
    pub stack: Vec<Vec<u8>>,
    pub alt_stack: Vec<Vec<u8>>,
    pub status: Status,
    pub executed_opcodes: Vec<OpCodes>,
    pub codeseparator_offset: usize,
}

impl State {
    /// Get a reference to the state's stack.
    #[must_use]
    pub fn stack(&self) -> &[Vec<u8>] {
        self.stack.as_ref()
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            r#"Stack: {:?}
Alt Stack: {:?}
Executed OpCodes: {:?}
        "#,
            self.stack
                .iter()
                .map(|data| data.to_hex())
                .collect::<Vec<String>>(),
            self.alt_stack
                .iter()
                .map(|data| data.to_hex())
                .collect::<Vec<String>>(),
            self.executed_opcodes
        )
    }
}
