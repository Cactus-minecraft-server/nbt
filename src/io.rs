use std::{
    collections::HashMap,
    io::{Error, ErrorKind, Read, Result, Write},
};

use crate::{Tag, TagId};

/// Binary reader for NBT format
pub struct Reader<R: Read> {
    inner: R,
}

impl<R: Read> Reader<R> {
    pub fn new(inner: R) -> Self {
        Reader { inner }
    }

    /// Read a full tag (ID + name + payload)
    pub fn read_tag(&mut self) -> Result<Tag> {
        let id = self.read_u8()?;
        if id == 0 {
            return Ok(Tag::End);
        }
        let name = Some(self.read_string()?);
        self.read_payload(id, name)
    }

    fn read_payload(&mut self, id: TagId, name: Option<String>) -> Result<Tag> {
        match id {
            1 => Ok(Tag::Byte {
                name,
                value: self.read_i8()?,
            }),
            2 => Ok(Tag::Short {
                name,
                value: self.read_i16()?,
            }),
            3 => Ok(Tag::Int {
                name,
                value: self.read_i32()?,
            }),
            4 => Ok(Tag::Long {
                name,
                value: self.read_i64()?,
            }),
            5 => Ok(Tag::Float {
                name,
                value: self.read_f32()?,
            }),
            6 => Ok(Tag::Double {
                name,
                value: self.read_f64()?,
            }),
            7 => {
                let len = self.read_i32()? as usize;
                let mut buf = vec![0u8; len];
                self.inner.read_exact(&mut buf)?;
                Ok(Tag::ByteArray { name, value: buf })
            }
            8 => {
                let s = self.read_string()?;
                Ok(Tag::String { name, value: s })
            }
            9 => {
                let elem_id = self.read_u8()?;
                let len = self.read_i32()? as usize;
                let mut elements = Vec::with_capacity(len);
                for _ in 0..len {
                    elements.push(self.read_payload(elem_id, None)?);
                }
                Ok(Tag::List {
                    name,
                    element_id: elem_id,
                    elements,
                })
            }
            10 => {
                let mut entries = HashMap::new();
                loop {
                    let id = self.read_u8()?;
                    if id == 0 {
                        break;
                    }
                    let key = self.read_string()?;
                    let tag = self.read_payload(id, Some(key.clone()))?;
                    entries.insert(key, tag);
                }
                Ok(Tag::Compound { name, entries })
            }
            11 => {
                let len = self.read_i32()? as usize;
                let mut v = Vec::with_capacity(len);
                for _ in 0..len {
                    v.push(self.read_i32()?);
                }
                Ok(Tag::IntArray { name, value: v })
            }
            12 => {
                let len = self.read_i32()? as usize;
                let mut v = Vec::with_capacity(len);
                for _ in 0..len {
                    v.push(self.read_i64()?);
                }
                Ok(Tag::LongArray { name, value: v })
            }
            other => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unknown tag id {}", other),
            )),
        }
    }

    fn read_u8(&mut self) -> Result<u8> {
        let mut buf = [0u8; 1];
        self.inner.read_exact(&mut buf)?;
        Ok(buf[0])
    }
    fn read_i8(&mut self) -> Result<i8> {
        Ok(self.read_u8()? as i8)
    }
    fn read_i16(&mut self) -> Result<i16> {
        let mut buf = [0u8; 2];
        self.inner.read_exact(&mut buf)?;
        Ok(i16::from_be_bytes(buf))
    }
    fn read_i32(&mut self) -> Result<i32> {
        let mut buf = [0u8; 4];
        self.inner.read_exact(&mut buf)?;
        Ok(i32::from_be_bytes(buf))
    }
    fn read_i64(&mut self) -> Result<i64> {
        let mut buf = [0u8; 8];
        self.inner.read_exact(&mut buf)?;
        Ok(i64::from_be_bytes(buf))
    }
    fn read_f32(&mut self) -> Result<f32> {
        Ok(f32::from_bits(self.read_i32()? as u32))
    }
    fn read_f64(&mut self) -> Result<f64> {
        Ok(f64::from_bits(self.read_i64()? as u64))
    }
    fn read_string(&mut self) -> Result<String> {
        let len = self.read_i16()? as usize;
        let mut buf = vec![0u8; len];
        self.inner.read_exact(&mut buf)?;
        String::from_utf8(buf).map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }
}

/// Binary writer for NBT format
pub struct Writer<W: Write> {
    inner: W,
}

impl<W: Write> Writer<W> {
    pub fn new(inner: W) -> Self {
        Writer { inner }
    }

    /// Write a full tag (ID + name + payload)
    pub fn write_tag(&mut self, tag: &Tag) -> Result<()> {
        let id = tag.id();
        self.write_u8(id)?;
        if id != 0 {
            if let Some(name) = tag.name() {
                self.write_string(name)?;
            } else {
                self.write_string("")?;
            }
            self.write_payload(tag)?;
        }
        Ok(())
    }

    fn write_payload(&mut self, tag: &Tag) -> Result<()> {
        match tag {
            Tag::End => {}
            Tag::Byte { value, .. } => self.write_i8(*value)?,
            Tag::Short { value, .. } => self.write_i16(*value)?,
            Tag::Int { value, .. } => self.write_i32(*value)?,
            Tag::Long { value, .. } => self.write_i64(*value)?,
            Tag::Float { value, .. } => self.write_f32(*value)?,
            Tag::Double { value, .. } => self.write_f64(*value)?,
            Tag::ByteArray { value, .. } => {
                self.write_i32(value.len() as i32)?;
                self.inner.write_all(value)?;
            }
            Tag::String { value, .. } => {
                self.write_string(value)?;
            }
            Tag::List {
                element_id,
                elements,
                ..
            } => {
                self.write_u8(*element_id)?;
                self.write_i32(elements.len() as i32)?;
                for elem in elements {
                    self.write_payload(elem)?; // lists omit names
                }
            }
            Tag::Compound { entries, .. } => {
                for (key, entry) in entries {
                    let id = entry.id();
                    self.write_u8(id)?;
                    self.write_string(key)?;
                    self.write_payload(entry)?;
                }
                self.write_u8(0)?; // TAG_End
            }
            Tag::IntArray { value, .. } => {
                self.write_i32(value.len() as i32)?;
                for &i in value {
                    self.write_i32(i)?;
                }
            }
            Tag::LongArray { value, .. } => {
                self.write_i32(value.len() as i32)?;
                for &l in value {
                    self.write_i64(l)?;
                }
            }
        }
        Ok(())
    }

    fn write_u8(&mut self, v: u8) -> Result<()> {
        self.inner.write_all(&[v])
    }
    fn write_i8(&mut self, v: i8) -> Result<()> {
        self.write_u8(v as u8)
    }
    fn write_i16(&mut self, v: i16) -> Result<()> {
        self.inner.write_all(&v.to_be_bytes())
    }
    fn write_i32(&mut self, v: i32) -> Result<()> {
        self.inner.write_all(&v.to_be_bytes())
    }
    fn write_i64(&mut self, v: i64) -> Result<()> {
        self.inner.write_all(&v.to_be_bytes())
    }
    fn write_f32(&mut self, v: f32) -> Result<()> {
        self.write_i32(v.to_bits() as i32)
    }
    fn write_f64(&mut self, v: f64) -> Result<()> {
        self.write_i64(v.to_bits() as i64)
    }
    fn write_string(&mut self, s: &str) -> Result<()> {
        let bytes = s.as_bytes();
        self.write_i16(bytes.len() as i16)?;
        self.inner.write_all(bytes)
    }
}
