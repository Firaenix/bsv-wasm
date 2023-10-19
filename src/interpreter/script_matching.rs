use std::ops::Neg;

use crate::{Hash, OpCodes, PublicKey, Script, ScriptBit, SigHash, SighashSignature, ToHex};
use num_bigint::{BigInt, Sign};

use super::{
    errors::InterpreterError,
    stack_trait::{self, ScriptStack},
    state::State,
    Interpreter, TxScript,
};

/// Script Matching functions
impl Interpreter {
    fn verify(boolean: bool) -> Result<(), InterpreterError> {
        match boolean {
            true => Ok(()),
            false => Err(InterpreterError::VerifyFailed),
        }
    }

    pub(crate) fn match_script_bit(&mut self, bit: &ScriptBit) -> Result<State, InterpreterError> {
        Ok(match bit {
            ScriptBit::OpCode(o) => match Interpreter::match_opcode(self.script_index, o, &mut self.state.clone(), self.tx_script.clone()) {
                Ok(mut next_state) => {
                    next_state.executed_opcodes.push(*o);
                    next_state
                }
                Err(e) => {
                    self.state.executed_opcodes.push(*o);
                    return Err(e);
                }
            },
            ScriptBit::Push(v) => {
                self.state.stack.push(v.clone());
                self.state.executed_opcodes.push(OpCodes::OP_DATA);
                self.state.clone()
            }
            ScriptBit::PushData(size, v) => {
                self.state.stack.push(v.clone());
                self.state.executed_opcodes.push(*size);
                self.state.clone()
            }
            ScriptBit::If { code, pass, fail } => {
                let predicate = self.state.stack.pop_bool()?;
                self.state.executed_opcodes.push(*code);

                if predicate {
                    let _removed: Vec<ScriptBit> = self.script_bits.splice(self.script_index + 1..self.script_index + 1, pass.clone()).collect();
                    // println!("Removed items: {:?}", removed);
                } else {
                    let _removed: Vec<ScriptBit> = self.script_bits.splice(self.script_index + 1..self.script_index + 1, fail.clone().unwrap_or_default()).collect();
                    // println!("Removed items: {:?}", removed);
                }

                self.state.clone()
            }
            ScriptBit::Coinbase(_) => todo!(),
        })
    }

    #[allow(unused_mut)]
    pub(crate) fn match_opcode(script_index: usize, opcode: &OpCodes, state: &mut State, tx: Option<TxScript>) -> Result<State, InterpreterError> {
        let mut state: &mut State = state;
        match opcode {
            OpCodes::OP_0 => state.stack.push_number(0)?,
            OpCodes::OP_1NEGATE => state.stack.push_number(-1)?,
            OpCodes::OP_1 => state.stack.push_number(1)?,
            OpCodes::OP_2 => state.stack.push_number(2)?,
            OpCodes::OP_3 => state.stack.push_number(3)?,
            OpCodes::OP_4 => state.stack.push_number(4)?,
            OpCodes::OP_5 => state.stack.push_number(5)?,
            OpCodes::OP_6 => state.stack.push_number(6)?,
            OpCodes::OP_7 => state.stack.push_number(7)?,
            OpCodes::OP_8 => state.stack.push_number(8)?,
            OpCodes::OP_9 => state.stack.push_number(9)?,
            OpCodes::OP_10 => state.stack.push_number(10)?,
            OpCodes::OP_11 => state.stack.push_number(11)?,
            OpCodes::OP_12 => state.stack.push_number(12)?,
            OpCodes::OP_13 => state.stack.push_number(13)?,
            OpCodes::OP_14 => state.stack.push_number(14)?,
            OpCodes::OP_15 => state.stack.push_number(15)?,
            OpCodes::OP_16 => state.stack.push_number(16)?,
            OpCodes::OP_NOP => {}
            OpCodes::OP_IF => {
                // NOP - handled by ScriptBit interpreter
            }
            OpCodes::OP_NOTIF => {
                // NOP - handled by ScriptBit interpreter
            }
            OpCodes::OP_ELSE => {
                // NOP - handled by ScriptBit interpreter
            }
            OpCodes::OP_ENDIF => {
                // NOP - handled by ScriptBit interpreter
            }
            OpCodes::OP_VERIFY => {
                let predicate = state.stack.pop_bool()?;
                Interpreter::verify(predicate)?
            }
            OpCodes::OP_RETURN => {
                return Ok(state.clone());
            }
            OpCodes::OP_TOALTSTACK => {
                let a = state.stack.pop_bytes()?;
                state.alt_stack.push_bytes(a);
            }
            OpCodes::OP_FROMALTSTACK => {
                let a = state.alt_stack.pop_bytes()?;
                state.stack.push_bytes(a);
            }
            OpCodes::OP_IFDUP => {
                let predicate = state.stack.pop_bool()?;
                if predicate {
                    let top_data = match state.stack.last().cloned() {
                        Some(v) => v,
                        None => return Err(InterpreterError::EmptyStack),
                    };

                    state.stack.push(top_data);
                }
            }
            OpCodes::OP_DEPTH => {
                let depth = state.stack.len();
                state.stack.push_number(depth as i64)?;
            }
            OpCodes::OP_DROP => {
                state.stack.pop_bytes()?;
            }
            OpCodes::OP_DUP => {
                let top_data = match state.stack.last().cloned() {
                    Some(v) => v,
                    None => return Err(InterpreterError::EmptyStack),
                };

                state.stack.push(top_data);
            }
            OpCodes::OP_NIP => {
                state.stack.remove(state.stack.len() - 2);
            }
            OpCodes::OP_OVER => {
                let index = state.stack.len() - 2;
                let second_last = state.stack.get(index).cloned().ok_or(InterpreterError::NumberOutOfRange)?;
                state.stack.push_bytes(second_last);
            }
            OpCodes::OP_PICK => {
                let index = state.stack.pop_number()?;
                let selected_item = state.stack.get((state.stack.len() - 1) - index as usize).cloned().ok_or(InterpreterError::NumberOutOfRange)?;
                state.stack.push_bytes(selected_item);
            }
            OpCodes::OP_ROLL => {
                let index = state.stack.pop_number()?;
                let selected_item = state.stack.remove((state.stack.len() - 1) - index as usize);
                state.stack.push_bytes(selected_item);
            }
            OpCodes::OP_ROT => {
                let len = state.stack.len();
                let third = state.stack.remove(len - 3);

                state.stack.push_bytes(third);
            }
            OpCodes::OP_SWAP => {
                let len = state.stack.len();
                state.stack.swap(len - 1, len - 2);
            }
            OpCodes::OP_TUCK => {
                let selected_item = state.stack.last().cloned().ok_or(InterpreterError::NumberOutOfRange)?;
                state.stack.insert(state.stack.len() - 2, selected_item);
            }
            OpCodes::OP_2DROP => {
                state.stack.pop_bytes()?;
                state.stack.pop_bytes()?;
            }
            OpCodes::OP_2DUP => {
                let first = state.stack.last().cloned().ok_or(InterpreterError::NumberOutOfRange)?;
                let second = state.stack.get(state.stack.len() - 2).cloned().ok_or(InterpreterError::NumberOutOfRange)?;

                state.stack.push_bytes(first);
                state.stack.push_bytes(second);
            }
            OpCodes::OP_3DUP => {
                let first = state.stack.last().cloned().ok_or(InterpreterError::NumberOutOfRange)?;
                let second = state.stack.get(state.stack.len() - 2).cloned().ok_or(InterpreterError::NumberOutOfRange)?;
                let third = state.stack.get(state.stack.len() - 3).cloned().ok_or(InterpreterError::NumberOutOfRange)?;

                state.stack.push_bytes(first);
                state.stack.push_bytes(second);
                state.stack.push_bytes(third);
            }
            OpCodes::OP_2OVER => {
                let len = state.stack.len();
                let third = state.stack[len - 3].clone();
                let fourth = state.stack[len - 4].clone();
                state.stack.push_bytes(fourth);
                state.stack.push_bytes(third);
            }
            OpCodes::OP_2ROT => {
                let index = state.stack.len() - 6;
                let sixth = state.stack.remove(index);
                let fifth = state.stack.remove(index);
                state.stack.push_bytes(sixth);
                state.stack.push_bytes(fifth);
            }

            OpCodes::OP_2SWAP => {
                let x1 = state.stack.pop_bytes()?;
                let x2 = state.stack.pop_bytes()?;
                let x3 = state.stack.pop_bytes()?;
                let x4 = state.stack.pop_bytes()?;

                state.stack.push_bytes(x3);
                state.stack.push_bytes(x4);
                state.stack.push_bytes(x1);
                state.stack.push_bytes(x2)
            }
            OpCodes::OP_CAT => {
                let mut x1 = state.stack.pop_bytes()?;
                let x2 = state.stack.pop_bytes()?;

                x1.extend_from_slice(&x2);

                state.stack.push_bytes(x1)
            }
            OpCodes::OP_SPLIT => {
                let x = state.stack.pop_bytes()?;
                let n = state.stack.pop_number()?;

                let (x1, x2) = x.split_at(n as usize);
                state.stack.push_bytes(x1.to_vec());
                state.stack.push_bytes(x2.to_vec());
            }

            OpCodes::OP_SIZE => {
                let len = state.stack.last().unwrap().len();
                state.stack.push_number(len as i64)?;
            }
            OpCodes::OP_INVERT => {
                let inverted_bytes = state.stack.pop_bytes()?.iter().map(|x| !x).collect();

                state.stack.push(inverted_bytes);
            }
            OpCodes::OP_AND => {
                let a = state.stack.pop_bytes()?;
                let b = state.stack.pop_bytes()?;

                let and_array = b.iter().zip(a.iter()).map(|(&x1, &x2)| x1 & x2).collect();

                state.stack.push_bytes(and_array);
            }
            OpCodes::OP_OR => {
                let a = state.stack.pop_bytes()?;
                let b = state.stack.pop_bytes()?;

                let or_array = b.iter().zip(a.iter()).map(|(&x1, &x2)| x1 | x2).collect();

                state.stack.push_bytes(or_array);
            }
            OpCodes::OP_XOR => {
                let a = state.stack.pop_bytes()?;
                let b = state.stack.pop_bytes()?;

                let xor_array = b.iter().zip(a.iter()).map(|(&x1, &x2)| x1 ^ x2).collect();

                state.stack.push_bytes(xor_array);
            }
            OpCodes::OP_EQUAL => {
                let a = state.stack.pop_bytes()?;
                let b = state.stack.pop_bytes()?;

                state.stack.push_bool(a.eq(&b))?;
            }
            OpCodes::OP_EQUALVERIFY => {
                let a = state.stack.pop_bytes()?;
                let b = state.stack.pop_bytes()?;

                Interpreter::verify(a.eq(&b))?;
            }
            OpCodes::OP_1ADD => {
                let a = state.stack.pop_bigint()?;

                state.stack.push_bigint(a + 1)?;
            }
            OpCodes::OP_1SUB => {
                let a = state.stack.pop_bigint()?;

                state.stack.push_bigint(a - 1)?;
            }
            OpCodes::OP_NEGATE => {
                let a = state.stack.pop_bigint()?;

                state.stack.push_bigint(-a)?;
            }
            OpCodes::OP_ABS => {
                let a = state.stack.pop_bigint()?;

                let pos = match a < BigInt::from(0) {
                    true => a.neg(),
                    false => a,
                };

                state.stack.push_bigint(pos)?;
            }
            OpCodes::OP_NOT => {
                let a = state.stack.pop_number()?;

                let notted = match a {
                    0 => 1,
                    _ => 0,
                };

                state.stack.push_number(notted)?;
            }
            OpCodes::OP_0NOTEQUAL => {
                let a = state.stack.pop_number()?;

                let notted = match a {
                    0 => 0,
                    _ => 1,
                };

                state.stack.push_number(notted)?;
            }
            OpCodes::OP_ADD => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                let sum = a + b;
                state.stack.push(sum.to_signed_bytes_le());
            }
            OpCodes::OP_SUB => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bigint(a - b)?;
            }
            OpCodes::OP_MUL => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bigint(a * b)?;
            }
            OpCodes::OP_DIV => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bigint(a / b)?;
            }
            OpCodes::OP_MOD => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bigint(a % b)?;
            }
            OpCodes::OP_LSHIFT => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_number()?;

                state.stack.push_bigint(a << b)?;
            }
            OpCodes::OP_RSHIFT => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_number()?;

                state.stack.push_bigint(a >> b)?;
            }
            OpCodes::OP_BOOLAND => {
                let a = state.stack.pop_bool()?;
                let b = state.stack.pop_bool()?;

                state.stack.push_bool(a && b)?;
            }
            OpCodes::OP_BOOLOR => {
                let a = state.stack.pop_bool()?;
                let b = state.stack.pop_bool()?;

                state.stack.push_bool(a || b)?;
            }
            OpCodes::OP_NUMEQUAL => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bool(a == b)?;
            }
            OpCodes::OP_NUMEQUALVERIFY => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                Interpreter::verify(a == b)?;
            }
            OpCodes::OP_NUMNOTEQUAL => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bool(a != b)?;
            }
            OpCodes::OP_LESSTHAN => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bool(a < b)?;
            }
            OpCodes::OP_LESSTHANOREQUAL => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bool(a <= b)?;
            }
            OpCodes::OP_GREATERTHAN => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bool(a > b)?;
            }
            OpCodes::OP_GREATERTHANOREQUAL => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                state.stack.push_bool(a >= b)?;
            }
            OpCodes::OP_MIN => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                let smallest = match a > b {
                    true => b,
                    false => a,
                };

                state.stack.push_bigint(smallest)?;
            }
            OpCodes::OP_MAX => {
                let a = state.stack.pop_bigint()?;
                let b = state.stack.pop_bigint()?;

                let biggest = match a < b {
                    true => b,
                    false => a,
                };

                state.stack.push_bigint(biggest)?;
            }
            OpCodes::OP_WITHIN => {
                let x = state.stack.pop_bigint()?;
                let min = state.stack.pop_bigint()?;
                let max = state.stack.pop_bigint()?;

                state.stack.push_bool(x >= min && x <= max)?;
            }
            OpCodes::OP_NUM2BIN => {
                let length = state.stack.pop_number()?;
                let bytes = state.stack.pop_bytes()?;

                if length < 1 || length < bytes.len() as i32 {
                    return Err(InterpreterError::InvalidStackOperation("OP_NUM2BIN failed, provide length was out of range"));
                }

                // Fill the data in, extend the buffer to the length of the length parameter
                let (sign, mut bin_array) = stack_trait::to_bigint(&bytes)?.to_bytes_le();
                bin_array.resize(length as usize, 0);
                let bin_array_len = bin_array.len();

                let full = bin_array[bin_array_len - 1] & 0x80;
                if full > 0 {
                    bin_array.push(0x00);
                }

                // // Add 0x00 to the end if last byte is positive sign
                match sign {
                    Sign::Plus => bin_array[bin_array_len - 1] |= 0x00,
                    Sign::Minus => bin_array[bin_array_len - 1] |= 0x80,
                    Sign::NoSign => return Err(InterpreterError::InvalidStackOperation("OP_NUM2BIN failed, invalid sign on bigint.")),
                };
                state.stack.push_bytes(bin_array);
            }
            OpCodes::OP_BIN2NUM => {
                let bigint = state.stack.pop_bigint()?;
                state.stack.push_bigint(bigint)?;
            }
            OpCodes::OP_RIPEMD160 => {
                let data = state.stack.pop_bytes()?;

                let result = Hash::ripemd_160(&data);
                state.stack.push(result.to_bytes());
            }
            OpCodes::OP_SHA1 => {
                let data = state.stack.pop_bytes()?;

                let result = Hash::sha_1(&data);
                state.stack.push(result.to_bytes());
            }
            OpCodes::OP_SHA256 => {
                let data = state.stack.pop_bytes()?;

                let result = Hash::sha_256(&data);
                state.stack.push(result.to_bytes());
            }
            OpCodes::OP_HASH160 => {
                let data = state.stack.pop_bytes()?;

                let result = Hash::hash_160(&data);
                state.stack.push(result.to_bytes());
            }
            OpCodes::OP_HASH256 => {
                let data = state.stack.pop_bytes()?;

                let result = Hash::sha_256d(&data);
                state.stack.push(result.to_bytes());
            }
            OpCodes::OP_CODESEPARATOR => state.codeseparator_offset = script_index + 1,
            OpCodes::OP_CHECKSIG => {
                let txscript = match tx {
                    Some(x) => x,
                    None => return Err(InterpreterError::RequiresTransaction(&OpCodes::OP_CHECKSIG)),
                };

                let is_signature_valid = checksig(state, txscript)?;
                state.stack.push_bool(is_signature_valid)?;
            }
            OpCodes::OP_CHECKSIGVERIFY => {
                let txscript = match tx {
                    Some(x) => x,
                    None => return Err(InterpreterError::RequiresTransaction(&OpCodes::OP_CHECKSIGVERIFY)),
                };

                let is_signature_valid = checksig(state, txscript)?;
                Interpreter::verify(is_signature_valid)?
            }
            OpCodes::OP_CHECKMULTISIG => {
                let mut txscript = match tx {
                    Some(x) => x,
                    None => return Err(InterpreterError::RequiresTransaction(&OpCodes::OP_CHECKMULTISIG)),
                };

                let is_multisig_valid = multisig(state, &mut txscript)?;
                state.stack.push_bool(is_multisig_valid)?
            }
            OpCodes::OP_CHECKMULTISIGVERIFY => {
                let mut txscript = match tx {
                    Some(x) => x,
                    None => return Err(InterpreterError::RequiresTransaction(&OpCodes::OP_CHECKMULTISIGVERIFY)),
                };

                let is_multisig_valid = multisig(state, &mut txscript)?;
                Interpreter::verify(is_multisig_valid)?
            }

            // TODO: Allow enabling of specific opcodes or opcode feature sets
            OpCodes::OP_CHECKLOCKTIMEVERIFY => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_CHECKLOCKTIMEVERIFY)),
            OpCodes::OP_CHECKSEQUENCEVERIFY => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_CHECKSEQUENCEVERIFY)),

            OpCodes::OP_VER => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_VER)),
            OpCodes::OP_VERIF => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_VERIF)),
            OpCodes::OP_VERNOTIF => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_VERNOTIF)),
            OpCodes::OP_RESERVED => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_RESERVED)),
            OpCodes::OP_RESERVED1 => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_RESERVED1)),
            OpCodes::OP_RESERVED2 => return Err(InterpreterError::DisabledOpCode(&OpCodes::OP_RESERVED2)),
            OpCodes::OP_NOP1 => {}
            OpCodes::OP_NOP4 => {}
            OpCodes::OP_NOP5 => {}
            OpCodes::OP_NOP6 => {}
            OpCodes::OP_NOP7 => {}
            OpCodes::OP_NOP8 => {}
            OpCodes::OP_NOP9 => {}
            OpCodes::OP_NOP10 => {}
            OpCodes::OP_2MUL => {
                let a = state.stack.pop_bigint()?;

                state.stack.push_bigint(a * 2)?;
            }
            OpCodes::OP_2DIV => {
                let a = state.stack.pop_bigint()?;

                state.stack.push_bigint(a / 2)?;
            }

            _ => return Err(InterpreterError::InvalidOpcode(*opcode)),
        };

        Ok(state.clone())
    }
}

fn checksig(state: &mut State, mut txscript: TxScript) -> Result<bool, InterpreterError> {
    let public_key = state.stack.pop_bytes()?;
    let signature = state.stack.pop_bytes()?;
    let sighash_byte = signature.last().cloned();
    println!("Sighash Byte: {:#?}", sighash_byte);
    let sighash = match sighash_byte {
        Some(x) => SigHash::try_from(x).map_err(|_| InterpreterError::FailedToConvertSighash)?,
        None => return Err(InterpreterError::InvalidStackOperation("could not read Sighash flag from signature")),
    };
    let preimage = calculate_sighash_preimage(&mut txscript, sighash, state.codeseparator_offset)?;
    let is_signature_valid = verify_tx_signature(&preimage, &mut txscript, &signature, &public_key)?;
    Ok(is_signature_valid)
}

fn multisig(state: &mut State, txscript: &mut TxScript) -> Result<bool, InterpreterError> {
    let pubkey_count = state.stack.pop_number()?;
    if pubkey_count < 1 {
        return Err(InterpreterError::InvalidStackOperation("PubKey count must be a positive number"));
    }

    // Slice the correct amount of pubkeys off the stack in reverse order so we can pop them.
    let mut pubkeys = state.stack.split_off(state.stack.len() - pubkey_count as usize);
    pubkeys.reverse();

    println!("Pubkeys: {:?}", pubkeys.iter().map(|x| x.to_hex()).collect::<Vec<String>>());

    let sig_count = state.stack.pop_number()?;
    if sig_count < 1 {
        return Err(InterpreterError::InvalidStackOperation("Signature count must be a positive number"));
    }

    if pubkey_count < sig_count {
        return Err(InterpreterError::InvalidStackOperation("PubKey count must be greater than or equal to Signature count"));
    }

    let sigs = state.stack.split_off(state.stack.len() - sig_count as usize);

    // Implement a bug to accidentally pop another item off the stack :-)
    state.stack.pop_bytes()?;

    let mut successes = 0;
    // Compare all Signatures against all public keys
    for sig in sigs {
        let sighash = match sig.last().cloned() {
            Some(x) => SigHash::try_from(x).map_err(|_| InterpreterError::FailedToConvertSighash)?,
            None => return Err(InterpreterError::InvalidStackOperation("could not read Sighash flag from signature")),
        };

        let preimage = calculate_sighash_preimage(txscript, sighash, state.codeseparator_offset)?;

        // Pop each pubkey because they are only to be compared against once.
        while let Some(public_key) = pubkeys.pop() {
            let is_signature_valid = verify_tx_signature(&preimage, txscript, &sig, &public_key)?;
            if is_signature_valid {
                successes += 1;
                break;
            }
        }
    }
    Ok(successes == sig_count)
}

fn verify_tx_signature(preimage: &[u8], txscript: &mut TxScript, signature: &[u8], public_key: &[u8]) -> Result<bool, InterpreterError> {
    let sighash_sig = SighashSignature::from_bytes_impl(signature, preimage)?;
    let is_signature_valid = txscript.tx.verify(&PublicKey::from_bytes_impl(public_key)?, &sighash_sig);
    Ok(is_signature_valid)
}

fn calculate_sighash_preimage(txscript: &mut TxScript, sighash: SigHash, codeseparator_offset: usize) -> Result<Vec<u8>, InterpreterError> {
    let txin = match txscript.tx.get_input(txscript.input_index) {
        Some(v) => v,
        _ => return Err(InterpreterError::InvalidStackOperation("could not get TxIn at the provided index")),
    };

    let unlock_script_len = txin.get_unlocking_script().to_script_bits().len();
    let script_offset = codeseparator_offset.saturating_sub(unlock_script_len);
    let unsigned_script = match txin.get_locking_script() {
        Some(v) => Script::from_script_bits(v.to_script_bits()[script_offset..].to_vec()),
        None => return Err(InterpreterError::InvalidStackOperation("TxIn at given index does not have locking script provided")),
    };
    println!("Unsigned script: {}", unsigned_script.to_asm_string());

    let satoshis = match txscript.tx.get_input(txscript.input_index).and_then(|x| x.get_satoshis()) {
        Some(v) => v,
        _ => return Err(InterpreterError::InvalidStackOperation("TxIn at given index does not have satoshis provided")),
    };

    txscript
        .tx
        .sighash_preimage_impl(txscript.input_index, sighash, &unsigned_script, satoshis)
        .map_err(|e| InterpreterError::SighashPreimageCalculation(e.to_string()))
}
