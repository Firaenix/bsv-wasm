use byteorder::LittleEndian;
use snafu::*;
use std::io::Cursor;

use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;


#[derive(Debug, Snafu)]
pub enum VarIntErrors {
  #[snafu(display("{}", error))]
  Failed { error: anyhow::Error },
}

pub trait VarInt {
  fn read_varint(&mut self) -> Result<u64, VarIntErrors>;
  fn write_varint(&mut self, varint: u64) -> Result<(), VarIntErrors>;
}

impl VarInt for Cursor<Vec<u8>> {
  fn read_varint(&mut self) -> Result<u64, VarIntErrors> {
    let read_result = match self.read_u8() {
      Ok(0xff) => self.read_u64::<LittleEndian>(),
      Ok(0xfe) => self.read_u32::<LittleEndian>().and_then(|x| Ok(x as u64)),
      Ok(0xfd) => self.read_u16::<LittleEndian>().and_then(|x| Ok(x as u64)),
      Ok(v) => Ok(v as u64),
      Err(e) => {
        return Err(VarIntErrors::Failed {
          error: anyhow::anyhow!(e),
        })
      }
    };

    match read_result {
      Err(e) => {
        return Err(VarIntErrors::Failed {
          error: anyhow::anyhow!(e),
        })
      }
      Ok(v) => Ok(v),
    }
  }

  /**
   * Borrowed from rust-sv by Brenton Gunning
   */
  fn write_varint(&mut self, varint: u64) -> Result<(), VarIntErrors> {
    let mut write = || {
      if varint <= 252 {
        self.write_u8(varint as u8)
      } else if varint <= 0xffff {
        self.write_u8(0xfd).and_then(|_| self.write_u16::<LittleEndian>(varint as u16))
      } else if varint <= 0xffffffff {
        self.write_u8(0xfe).and_then(|_| self.write_u32::<LittleEndian>(varint as u32))
      } else {
        self.write_u8(0xff).and_then(|_| self.write_u64::<LittleEndian>(varint))
      }
    };
    
    match write() {
      Err(e) => return Err(VarIntErrors::Failed {
        error: anyhow::anyhow!(e),
      }),
      Ok(_) => Ok(())
    }
  }
}
