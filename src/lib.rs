use std::collections::HashMap;
mod test;
// Id of nbt
pub type TagId = u8;

// All the types of nbt
#[derive(PartialEq,Clone,Debug,)]
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
    pub fn new_ByteArray(name: impl Into<String>, v: Vec<u8>) -> Tag {
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

