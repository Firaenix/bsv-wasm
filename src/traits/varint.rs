use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use std::io::Cursor;
use std::io::Result;

use crate::OpCodes;

pub trait VarIntReader {
    fn read_varint(&mut self) -> Result<u64>;
}

pub trait VarIntWriter {
    fn write_varint(&mut self, varint: u64) -> std::io::Result<usize>;
}

pub struct VarInt {}

impl VarInt {
    pub fn get_varint_size(data_length: u64) -> usize {
        if data_length <= 252 {
            1
        } else if data_length <= 0xffff {
            2
        } else if data_length <= 0xffffffff {
            4
        } else {
            8
        }
    }

    pub fn get_pushdata_opcode(length: u64) -> Option<OpCodes> {
        if length <= 252 {
            None
        } else if length <= 0xff {
            Some(OpCodes::OP_PUSHDATA1)
        } else if length <= 0xffff {
            Some(OpCodes::OP_PUSHDATA2)
        } else {
            Some(OpCodes::OP_PUSHDATA4)
        }
    }

    pub fn get_varint_bytes(length: u64) -> Vec<u8> {
        if length <= 252 {
            vec![length as u8]
        } else if length <= 0xff {
            let mut push1 = vec![0xfd];
            push1.extend((length as u16).to_le_bytes());
            push1
        } else if length <= 0xffff {
            let mut push2 = vec![0xfe];
            push2.extend((length as u32).to_le_bytes());
            push2
        } else {
            let mut push4 = vec![0xff];
            push4.extend((length as u64).to_le_bytes());
            push4
        }
    }
}

impl VarIntReader for Cursor<Vec<u8>> {
    fn read_varint(&mut self) -> Result<u64> {
        match self.read_u8() {
            Ok(0xff) => self.read_u64::<LittleEndian>(),
            Ok(0xfe) => self.read_u32::<LittleEndian>().map(|x| x as u64),
            Ok(0xfd) => self.read_u16::<LittleEndian>().map(|x| x as u64),
            Ok(v) => Ok(v as u64),
            Err(e) => Err(e),
        }
    }
}

impl VarIntWriter for Cursor<Vec<u8>> {
    /**
     * Borrowed from rust-sv by Brenton Gunning
     */
    fn write_varint(&mut self, varint: u64) -> Result<usize> {
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

        write()?;
        Ok(varint as usize)
    }
}

impl VarIntReader for Vec<u8> {
    fn read_varint(&mut self) -> Result<u64> {
        let mut cursor = Cursor::new(&self);

        match cursor.read_u8() {
            Ok(0xff) => cursor.read_u64::<LittleEndian>(),
            Ok(0xfe) => cursor.read_u32::<LittleEndian>().map(|x| x as u64),
            Ok(0xfd) => cursor.read_u16::<LittleEndian>().map(|x| x as u64),
            Ok(v) => Ok(v as u64),
            Err(e) => Err(e),
        }
    }
}

impl VarIntWriter for Vec<u8> {
    /**
     * Borrowed from rust-sv by Brenton Gunning
     */
    fn write_varint(&mut self, varint: u64) -> Result<usize> {
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

        write()?;
        Ok(varint as usize)
    }
}

impl VarIntReader for Cursor<&'_ [u8]> {
    fn read_varint(&mut self) -> Result<u64> {
        match self.read_u8() {
            Ok(0xff) => self.read_u64::<LittleEndian>(),
            Ok(0xfe) => self.read_u32::<LittleEndian>().map(|x| x as u64),
            Ok(0xfd) => self.read_u16::<LittleEndian>().map(|x| x as u64),
            Ok(v) => Ok(v as u64),
            Err(e) => Err(e),
        }
    }
}
