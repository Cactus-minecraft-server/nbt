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

// ---------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use std::io::{Cursor, Read};

    use flate2::bufread::GzDecoder;

    use crate::{Reader, Tag, Writer};

    fn sample_level_compound() -> Tag {
        use std::collections::HashMap;
        let mut entries = HashMap::new();
        entries.insert(
            "DataVersion".into(),
            Tag::Int {
                name: Some("DataVersion".into()),
                value: 3837,
            },
        );
        entries.insert(
            "LevelName".into(),
            Tag::String {
                name: Some("LevelName".into()),
                value: "test".into(),
            },
        );
        entries.insert(
            "GameType".into(),
            Tag::Int {
                name: Some("GameType".into()),
                value: 0,
            },
        );
        entries.insert(
            "Difficulty".into(),
            Tag::Byte {
                name: Some("Difficulty".into()),
                value: 2,
            },
        );
        entries.insert(
            "hardcore".into(),
            Tag::Byte {
                name: Some("hardcore".into()),
                value: 0,
            },
        );
        entries.insert(
            "allowCommands".into(),
            Tag::Byte {
                name: Some("allowCommands".into()),
                value: 1,
            },
        );
        entries.insert(
            "SpawnX".into(),
            Tag::Int {
                name: Some("SpawnX".into()),
                value: 0,
            },
        );
        entries.insert(
            "SpawnY".into(),
            Tag::Int {
                name: Some("SpawnY".into()),
                value: 64,
            },
        );
        entries.insert(
            "SpawnZ".into(),
            Tag::Int {
                name: Some("SpawnZ".into()),
                value: 0,
            },
        );

        let list = Tag::List {
            name: Some("TestList".into()),
            element_id: 3,
            elements: vec![
                Tag::Int {
                    name: None,
                    value: 1,
                },
                Tag::Int {
                    name: None,
                    value: 2,
                },
                Tag::Int {
                    name: None,
                    value: 3,
                },
            ],
        };
        entries.insert("TestList".into(), list);
        Tag::Compound {
            name: Some("Data".into()),
            entries,
        }
    }

    fn write_uncompressed(tag: &Tag) -> Vec<u8> {
        let mut buf = Cursor::new(Vec::new());
        let mut w = Writer::new(&mut buf);
        w.write_tag(tag).unwrap();
        buf.into_inner()
    }

    fn read_uncompressed(bytes: &[u8]) -> Tag {
        let mut r = Reader::new(Cursor::new(bytes));
        r.read_tag().unwrap()
    }

    #[test]
    fn roundtrip_uncompressed() {
        let root = sample_level_compound();
        let bytes = write_uncompressed(&root);
        let back = read_uncompressed(&bytes);

        match back {
            Tag::Compound { name, .. } => assert_eq!(name.as_deref(), Some("Data")),
            _ => panic!("root not a Compound"),
        }
    }

    #[test]
    fn list_elements_have_no_names() {
        let root = sample_level_compound();
        let bytes = write_uncompressed(&root);
        let back = read_uncompressed(&bytes);
        let Tag::Compound { entries, .. } = back else {
            panic!("not compound");
        };
        let Tag::List {
            element_id,
            elements,
            ..
        } = entries.get("TestList").expect("missing TestList")
        else {
            panic!("not list");
        };
        assert_eq!(*element_id, 3);
        assert_eq!(elements.len(), 3);
        for e in elements {
            match e {
                Tag::Int { name, value } => {
                    assert!(name.is_none());
                    assert!(*value >= 1 && *value <= 3);
                }
                _ => panic!("list element not Int"),
            }
        }
    }

    #[test]
    fn string_length_u16_boundary() {
        let s = "a".repeat(300);
        let tag = Tag::String {
            name: Some("S".into()),
            value: s.clone(),
        };
        let root = Tag::Compound {
            name: Some("Data".into()),
            entries: {
                let mut m = std::collections::HashMap::new();
                m.insert("S".into(), tag);
                m
            },
        };
        let bytes = write_uncompressed(&root);
        let back = read_uncompressed(&bytes);
        let Tag::Compound { entries, .. } = back else {
            panic!("not compound");
        };
        let Tag::String { value, .. } = entries.get("S").expect("missing S") else {
            panic!("not string");
        };
        assert_eq!(value, &s);
    }

    #[test]
    fn roundtrip_gzip() {
        let root = sample_level_compound();

        let mut out = Vec::new();
        {
            let cursor = Cursor::new(&mut out);
            let mut w = Writer::to_gzip(cursor);
            w.write_tag(&root).unwrap();
        }

        assert!(out.len() >= 2);
        assert_eq!(out[0], 0x1f);
        assert_eq!(out[1], 0x8b);

        let mut r = Reader::from_gzip(Cursor::new(&out));
        let back = r.read_tag().unwrap();
        match back {
            Tag::Compound { name, .. } => assert_eq!(name.as_deref(), Some("Data")),
            _ => panic!("root not a Compound"),
        }
    }

    #[test]
    fn error_on_unknown_tag_id() {
        let mut raw = Vec::new();
        raw.push(99u8);
        raw.extend_from_slice(&0u16.to_be_bytes());
        let mut r = Reader::new(Cursor::new(raw));
        let err = r.read_tag().err().expect("should error");
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
    }

    #[test]
    fn compound_end_marker_required() {
        let mut raw = Vec::new();
        raw.push(10u8);
        raw.extend_from_slice(&(4u16.to_be_bytes()));
        raw.extend_from_slice(b"Data");

        let mut r = Reader::new(Cursor::new(raw));
        let res = r.read_tag();
        assert!(res.is_err(), "Reader should fail on missing TAG_End");
    }

    #[test]
    fn float_and_double_are_be() {
        let mut entries = std::collections::HashMap::new();
        entries.insert(
            "F".into(),
            Tag::Float {
                name: Some("F".into()),
                value: 1234.5f32,
            },
        );
        entries.insert(
            "D".into(),
            Tag::Double {
                name: Some("D".into()),
                value: -0.25f64,
            },
        );
        let root = Tag::Compound {
            name: Some("Data".into()),
            entries,
        };

        let bytes = write_uncompressed(&root);

        let back = read_uncompressed(&bytes);
        let Tag::Compound { entries, .. } = back else {
            panic!("not compound");
        };
        let Tag::Float { value: f, .. } = entries.get("F").unwrap() else {
            panic!("F missing");
        };
        let Tag::Double { value: d, .. } = entries.get("D").unwrap() else {
            panic!("D missing");
        };
        assert!((*f - 1234.5).abs() < 1e-4);
        assert!((*d + 0.25).abs() < 1e-12);
    }

    #[test]
    fn int_long_arrays_roundtrip() {
        let ia = (0..16).map(|i| i - 8).collect::<Vec<_>>();
        let la = (0..8).map(|i| (i as i64) * (1 << 33)).collect::<Vec<_>>();
        let mut entries = std::collections::HashMap::new();
        entries.insert(
            "IA".into(),
            Tag::IntArray {
                name: Some("IA".into()),
                value: ia.clone(),
            },
        );
        entries.insert(
            "LA".into(),
            Tag::LongArray {
                name: Some("LA".into()),
                value: la.clone(),
            },
        );
        let root = Tag::Compound {
            name: Some("Data".into()),
            entries,
        };

        let bytes = write_uncompressed(&root);
        let back = read_uncompressed(&bytes);

        let Tag::Compound { entries, .. } = back else {
            panic!("not compound");
        };
        let Tag::IntArray { value: ia2, .. } = entries.get("IA").unwrap() else {
            panic!("IA");
        };
        let Tag::LongArray { value: la2, .. } = entries.get("LA").unwrap() else {
            panic!("LA");
        };
        assert_eq!(ia2, &ia);
        assert_eq!(la2, &la);
    }

    #[test]
    fn gzip_stream_is_valid_by_gzdecoder() {
        let root = sample_level_compound();
        let mut out = Vec::new();
        {
            let cursor = Cursor::new(&mut out);
            let mut w = Writer::to_gzip(cursor);
            w.write_tag(&root).unwrap();
        }
        let mut dec = GzDecoder::new(Cursor::new(&out));
        let mut raw = Vec::new();
        dec.read_to_end(&mut raw).unwrap();
        assert!(!raw.is_empty());

        assert_eq!(raw[0], 10u8);
    }
}
