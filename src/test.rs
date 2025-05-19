// src/test.rs

use crate::{Tag, read_nbt, write_nbt};

#[test]
fn new_byte_tag() {
    let tag = Tag::new_byte("health", 20);
    match tag {
        Tag::Byte { name, value } => {
            assert_eq!(name.unwrap(), "health");
            assert_eq!(value, 20);
        }
        _ => panic!("Expected Byte tag"),
    }
}

#[test]
fn new_short_tag() {
    let tag = Tag::new_short("optLevel", 3);
    match tag {
        Tag::Short { name, value } => {
            assert_eq!(name.unwrap(), "optLevel");
            assert_eq!(value, 3);
        }
        _ => panic!("Expected Short tag"),
    }
}

#[test]
fn new_int_tag() {
    let tag = Tag::new_int("xPos", 100);
    match tag {
        Tag::Int { name, value } => {
            assert_eq!(name.unwrap(), "xPos");
            assert_eq!(value, 100);
        }
        _ => panic!("Expected Int tag"),
    }
}

#[test]
fn new_long_tag() {
    let tag = Tag::new_long("timestamp", 1_620_000_000_000);
    match tag {
        Tag::Long { name, value } => {
            assert_eq!(name.unwrap(), "timestamp");
            assert_eq!(value, 1_620_000_000_000);
        }
        _ => panic!("Expected Long tag"),
    }
}

#[test]
fn new_float_tag() {
    let tag = Tag::new_float("speed", 0.5);
    match tag {
        Tag::Float { name, value } => {
            assert_eq!(name.unwrap(), "speed");
            assert!((value - 0.5).abs() < f32::EPSILON);
        }
        _ => panic!("Expected Float tag"),
    }
}

#[test]
fn new_double_tag() {
    let tag = Tag::new_double("gravity", 9.81);
    match tag {
        Tag::Double { name, value } => {
            assert_eq!(name.unwrap(), "gravity");
            assert!((value - 9.81).abs() < f64::EPSILON);
        }
        _ => panic!("Expected Double tag"),
    }
}

#[test]
fn new_byte_array_tag() {
    let data = vec![1u8, 2, 3, 4];
    let tag = Tag::new_byte_array("blocks", data.clone());
    match tag {
        Tag::ByteArray { name, value } => {
            assert_eq!(name.unwrap(), "blocks");
            assert_eq!(value, data);
        }
        _ => panic!("Expected ByteArray tag"),
    }
}

#[test]
fn new_string_tag() {
    let tag = Tag::new_string("name", "Steve");
    match tag {
        Tag::String { name, value } => {
            assert_eq!(name.unwrap(), "name");
            assert_eq!(value, "Steve");
        }
        _ => panic!("Expected String tag"),
    }
}

#[test]
fn new_list_tag() {
    let elements = vec![Tag::new_int("", 1), Tag::new_int("", 2)];
    let tag = Tag::new_list("nums", 3, elements.clone());
    match tag {
        Tag::List {
            name,
            element_id,
            elements: elems,
        } => {
            assert_eq!(name.unwrap(), "nums");
            assert_eq!(element_id, 3);
            assert_eq!(elems, elements);
        }
        _ => panic!("Expected List tag"),
    }
}

#[test]
fn new_compound_tag() {
    let tag = Tag::new_compound("Level");
    match tag {
        Tag::Compound { name, entries } => {
            assert_eq!(name.unwrap(), "Level");
            assert!(entries.is_empty());
        }
        _ => panic!("Expected Compound tag"),
    }
}

#[test]
fn new_int_array_tag() {
    let data = vec![1i32, 2, 3];
    let tag = Tag::new_int_array("palette", data.clone());
    match tag {
        Tag::IntArray { name, value } => {
            assert_eq!(name.unwrap(), "palette");
            assert_eq!(value, data);
        }
        _ => panic!("Expected IntArray tag"),
    }
}

#[test]
fn new_long_array_tag() {
    let data = vec![100i64, 200];
    let tag = Tag::new_long_array("timestamps", data.clone());
    match tag {
        Tag::LongArray { name, value } => {
            assert_eq!(name.unwrap(), "timestamps");
            assert_eq!(value, data);
        }
        _ => panic!("Expected LongArray tag"),
    }
}

#[test]
fn roundtrip_compound() {
    let mut root = Tag::new_compound("Test");
    root.insert("value".to_string(), Tag::new_int("value", 42));

    let mut buf = Vec::new();
    write_nbt(&root, &mut buf).unwrap();

    let decoded = read_nbt(&buf[..]).unwrap();
    assert_eq!(decoded, root);
}
