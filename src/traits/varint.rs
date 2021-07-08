use byteorder::LittleEndian;
use byteorder::ReadBytesExt;
use byteorder::WriteBytesExt;
use snafu::*;
use std::io::Cursor;
use std::io::Result;

pub trait VarInt {
    fn read_varint(&mut self) -> Result<u64>;
    fn write_varint(&mut self, varint: u64) -> std::io::Result<()>;
}

impl VarInt for Cursor<Vec<u8>> {
    fn read_varint(&mut self) -> Result<u64> {
        match self.read_u8() {
            Ok(0xff) => self.read_u64::<LittleEndian>(),
            Ok(0xfe) => self.read_u32::<LittleEndian>().map(|x| x as u64),
            Ok(0xfd) => self.read_u16::<LittleEndian>().map(|x| x as u64),
            Ok(v) => Ok(v as u64),
            Err(e) => Err(e),
        }
    }

    /**
     * Borrowed from rust-sv by Brenton Gunning
     */
    fn write_varint(&mut self, varint: u64) -> Result<()> {
        let mut write = || {
            if varint <= 252 {
                self.write_u8(varint as u8)
            } else if varint <= 0xffff {
                self.write_u8(0xfd)
                    .and_then(|_| self.write_u16::<LittleEndian>(varint as u16))
            } else if varint <= 0xffffffff {
                self.write_u8(0xfe)
                    .and_then(|_| self.write_u32::<LittleEndian>(varint as u32))
            } else {
                self.write_u8(0xff)
                    .and_then(|_| self.write_u64::<LittleEndian>(varint))
            }
        };

        write()
    }
}

impl VarInt for Vec<u8> {
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

    /**
     * Borrowed from rust-sv by Brenton Gunning
     */
    fn write_varint(&mut self, varint: u64) -> Result<()> {
        let mut write = || {
            if varint <= 252 {
                self.write_u8(varint as u8)
            } else if varint <= 0xffff {
                self.write_u8(0xfd)
                    .and_then(|_| self.write_u16::<LittleEndian>(varint as u16))
            } else if varint <= 0xffffffff {
                self.write_u8(0xfe)
                    .and_then(|_| self.write_u32::<LittleEndian>(varint as u32))
            } else {
                self.write_u8(0xff)
                    .and_then(|_| self.write_u64::<LittleEndian>(varint))
            }
        };

        write()
    }
}
