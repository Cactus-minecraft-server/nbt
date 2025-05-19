// src/lib.rs

use std::collections::HashMap;
pub mod io;
mod test;
pub use io::{Reader, Writer};

/// Identifier for an NBT tag type
pub type TagId = u8;

/// All NBT tag types
#[derive(PartialEq, Clone, Debug)]
pub enum Tag {
    End,
    Byte {
        name: Option<String>,
        value: i8,
    },
    Short {
        name: Option<String>,
        value: i16,
    },
    Int {
        name: Option<String>,
        value: i32,
    },
    Long {
        name: Option<String>,
        value: i64,
    },
    Float {
        name: Option<String>,
        value: f32,
    },
    Double {
        name: Option<String>,
        value: f64,
    },
    ByteArray {
        name: Option<String>,
        value: Vec<u8>,
    },
    String {
        name: Option<String>,
        value: String,
    },
    List {
        name: Option<String>,
        element_id: TagId,
        elements: Vec<Tag>,
    },
    Compound {
        name: Option<String>,
        entries: HashMap<String, Tag>,
    },
    IntArray {
        name: Option<String>,
        value: Vec<i32>,
    },
    LongArray {
        name: Option<String>,
        value: Vec<i64>,
    },
}

impl Tag {
    pub fn new_byte(name: impl Into<String>, v: i8) -> Tag {
        Tag::Byte {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_short(name: impl Into<String>, v: i16) -> Tag {
        Tag::Short {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_int(name: impl Into<String>, v: i32) -> Tag {
        Tag::Int {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_long(name: impl Into<String>, v: i64) -> Tag {
        Tag::Long {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_float(name: impl Into<String>, v: f32) -> Tag {
        Tag::Float {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_double(name: impl Into<String>, v: f64) -> Tag {
        Tag::Double {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_byte_array(name: impl Into<String>, v: Vec<u8>) -> Tag {
        Tag::ByteArray {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_string(name: impl Into<String>, v: impl Into<String>) -> Tag {
        Tag::String {
            name: Some(name.into()),
            value: v.into(),
        }
    }
    pub fn new_list(name: impl Into<String>, element_id: TagId, elements: Vec<Tag>) -> Tag {
        Tag::List {
            name: Some(name.into()),
            element_id,
            elements,
        }
    }
    pub fn new_compound(name: impl Into<String>) -> Tag {
        Tag::Compound {
            name: Some(name.into()),
            entries: HashMap::new(),
        }
    }
    pub fn new_int_array(name: impl Into<String>, v: Vec<i32>) -> Tag {
        Tag::IntArray {
            name: Some(name.into()),
            value: v,
        }
    }
    pub fn new_long_array(name: impl Into<String>, v: Vec<i64>) -> Tag {
        Tag::LongArray {
            name: Some(name.into()),
            value: v,
        }
    }
}

impl Tag {
    /// Insert a sub-tag into a Compound
    pub fn insert(&mut self, key: String, tag: Tag) {
        if let Tag::Compound { entries, .. } = self {
            entries.insert(key, tag);
        } else {
            panic!("insert() called on non-Compound");
        }
    }

    /// Retrieve a sub-tag from a Compound
    pub fn get(&self, key: &str) -> Option<&Tag> {
        if let Tag::Compound { entries, .. } = self {
            entries.get(key)
        } else {
            None
        }
    }

    /// Add an element to a List
    pub fn push(&mut self, tag: Tag) {
        if let Tag::List { elements, .. } = self {
            elements.push(tag);
        } else {
            panic!("push() called on non-List");
        }
    }

    /// Numeric ID of this tag (0 = End, 1 = Byte, …, 12 = LongArray)
    pub fn id(&self) -> TagId {
        match self {
            Tag::End => 0,
            Tag::Byte { .. } => 1,
            Tag::Short { .. } => 2,
            Tag::Int { .. } => 3,
            Tag::Long { .. } => 4,
            Tag::Float { .. } => 5,
            Tag::Double { .. } => 6,
            Tag::ByteArray { .. } => 7,
            Tag::String { .. } => 8,
            Tag::List { .. } => 9,
            Tag::Compound { .. } => 10,
            Tag::IntArray { .. } => 11,
            Tag::LongArray { .. } => 12,
        }
    }

    /// Name of this tag (None for End or list elements)
    pub fn name(&self) -> Option<&str> {
        match self {
            Tag::End => None,
            Tag::Byte { name, .. }
            | Tag::Short { name, .. }
            | Tag::Int { name, .. }
            | Tag::Long { name, .. }
            | Tag::Float { name, .. }
            | Tag::Double { name, .. }
            | Tag::ByteArray { name, .. }
            | Tag::String { name, .. }
            | Tag::List { name, .. }
            | Tag::Compound { name, .. }
            | Tag::IntArray { name, .. }
            | Tag::LongArray { name, .. } => name.as_deref(),
        }
    }
}

/// Read an NBT Tag from any reader
pub fn read_nbt<R: std::io::Read>(reader: R) -> std::io::Result<Tag> {
    Reader::new(reader).read_tag()
}

/// Write an NBT Tag to any writer
pub fn write_nbt<W: std::io::Write>(tag: &Tag, writer: W) -> std::io::Result<()> {
    Writer::new(writer).write_tag(tag)
}
