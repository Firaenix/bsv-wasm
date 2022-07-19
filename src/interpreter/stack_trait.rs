use num_bigint::{BigInt, Sign};

use super::{errors::InterpreterError};

pub trait ScriptStack {
    fn push_bytes(&mut self, data: Vec<u8>);
    fn push_number(&mut self, val: i64) -> Result<(), InterpreterError>;
    fn push_bigint(&mut self, bigint: BigInt) -> Result<(), InterpreterError>;
    fn push_bool(&mut self, boolean: bool) -> Result<(), InterpreterError>;

    fn pop_bytes(&mut self) -> Result<Vec<u8>, InterpreterError>;
    fn pop_number(&mut self) -> Result<i32, InterpreterError>;
    fn pop_bigint(&mut self) -> Result<BigInt, InterpreterError>;
    fn pop_bool(&mut self) -> Result<bool, InterpreterError>;
}

pub fn to_bigint(data: &[u8]) -> Result<BigInt, InterpreterError> {
    let mut data = data.to_vec();
    let length = data.len();

    if length == 0 {
        return Ok(BigInt::from(0_usize))
    }

    let mut sign = Sign::Plus;
    if data[length - 1] & 0x80 == 0x80 {
        sign = Sign::Minus;
    }
    data[length - 1] &= !0x80;
    Ok(BigInt::from_bytes_le(sign, &data))
}

impl ScriptStack for Vec<Vec<u8>> {
    fn push_bytes(&mut self, data: Vec<u8>) {
        self.push(data)
    }

    fn pop_bytes(&mut self) -> Result<Vec<u8>, InterpreterError> {
        Ok(self.pop().ok_or_else(|| InterpreterError::EmptyStack)?)
    }

    fn push_number(&mut self, val: i64) -> Result<(), InterpreterError> {
        // Range: [-2^31+1, 2^31-1]
        if val > i32::MAX as i64 || val < i32::MIN  as i64 {
            return Err(InterpreterError::NumberOutOfRange);
        }
        let (posval, negmask) = if val < 0 { (-val, 128) } else { (val, 0) };
        if posval == 0 {
            self.push(vec![])
        } else if posval < 128 {
            self.push(vec![(posval as u8) | negmask])
        } else if posval < 32768 {
            self.push(vec![(posval >> 0) as u8, ((posval >> 8) as u8) | negmask])
        } else if posval < 8388608 {
            self.push(vec![
                (posval >> 0) as u8,
                (posval >> 8) as u8,
                ((posval >> 16) as u8) | negmask,
            ])
        } else {
            self.push(vec![
                (posval >> 0) as u8,
                (posval >> 8) as u8,
                (posval >> 16) as u8,
                ((posval >> 24) as u8) | negmask,
            ])
        }

        Ok(())
    }

    fn push_bigint(&mut self, bigint: BigInt) -> Result<(), InterpreterError> {
        let (sign, mut bytes) = bigint.to_bytes_le();
        if bytes[bytes.len() - 1] & 0x80 == 0x80 {
            bytes.push(match sign {
                Sign::Plus | Sign::NoSign => 0x00,
                Sign::Minus => 0x80,
            });
        } else if sign == Sign::Minus {
            let len = bytes.len();
            bytes[len - 1] |= 0x80;
        }
        if bytes.len() == 1 && bytes[0] == 0 {
            self.push(vec![]);
        } else {
            self.push(bytes);
        }

        Ok(())
    }

    fn pop_bigint(&mut self) -> Result<BigInt, InterpreterError> {
        let data = self.pop().ok_or_else(|| InterpreterError::EmptyStack)?;
        to_bigint(&data)
    } 

    fn pop_bool(&mut self) -> Result<bool, InterpreterError> {
        let data = self.pop().ok_or_else(|| InterpreterError::EmptyStack)?;

        if data.len() > 4 {
            return Err(InterpreterError::TooLongForBool);
        }

        Ok(BigInt::from_signed_bytes_le(&data) >= BigInt::from_slice(num_bigint::Sign::Plus, &[1]))
    }

    fn push_bool(&mut self, boolean: bool) -> Result<(), InterpreterError> {
        let data = match boolean {
            true => vec![1],
            false => vec![0],
        };

        self.push(data);
        Ok(())
    }

    fn pop_number(&mut self) -> Result<i32, InterpreterError> {
        let bytes = self.pop_bytes()?;
        // Numbers cannot be popped having more than 4 bytes, but may overflow on the stack to 5 bytes
        // after certain operations and may be used as byte vectors.
        if bytes.len() > 4 {
            // let msg = format!("Cannot pop num, len too long {}", top.len());
            return Err(InterpreterError::NumberOutOfRange);
        }
        
        let mut val = match bytes.len() {
            0 => return Ok(0),
            1 => (bytes[0] & 127) as i64,
            2 => (((bytes[1] & 127) as i64) << 8) + ((bytes[0] as i64) << 0),
            3 => (((bytes[2] & 127) as i64) << 16) + ((bytes[1] as i64) << 8) + ((bytes[0] as i64) << 0),
            4 => {
                (((bytes[3] & 127) as i64) << 24)
                    + ((bytes[2] as i64) << 16)
                    + ((bytes[1] as i64) << 8)
                    + ((bytes[0] as i64) << 0)
            }
            _ => {
                for i in 4..bytes.len() - 1 {
                    if bytes[i] != 0 {
                        return Err(InterpreterError::NumberOutOfRange);
                    }
                }
                if bytes[bytes.len() - 1] & 127 != 0 {
                    return Err(InterpreterError::NumberOutOfRange);
                }
                ((bytes[3] as i64) << 24)
                    + ((bytes[2] as i64) << 16)
                    + ((bytes[1] as i64) << 8)
                    + ((bytes[0] as i64) << 0)
            }
        };
        if bytes[bytes.len() - 1] & 128 != 0 {
            val = 0 - val;
        }

        Ok(val as i32)
    }
}
