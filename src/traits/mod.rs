use byteorder::LittleEndian;
use std::io::Cursor;
use snafu::*;

use byteorder::ReadBytesExt;

pub trait HexTrait {
  fn to_hex(&self) -> String;
}

#[derive(Debug, Snafu)]
pub enum VarIntErrors {
  #[snafu(display("{}", error))]
  Failed{
    error: anyhow::Error
  },
}

pub trait VarIntReader {
  fn read_varint(&mut self) -> Result<u64, VarIntErrors>;
}

impl VarIntReader for Cursor<Vec<u8>> {
  fn read_varint(&mut self) -> Result<u64, VarIntErrors> { 
    let read_result = match self.read_u8() {
      Ok(0xff) => self.read_u64::<LittleEndian>(),
      Ok(0xfe) => self.read_u32::<LittleEndian>().and_then(|x| Ok(x as u64)),
      Ok(0xfd) => self.read_u16::<LittleEndian>().and_then(|x| Ok(x as u64)),
      Ok(v) => Ok(v as u64),
      Err(e) => return Err(VarIntErrors::Failed{ error: anyhow::anyhow!(e) })
    };

    match read_result {
      Err(e) => return Err(VarIntErrors::Failed{ error: anyhow::anyhow!(e) }),
      Ok(v) => Ok(v)
    }
  }
}