use std::collections::{BTreeMap};
use lazy_static::lazy_static;

const UTF8_DATA: &[u8] = include_bytes!("cp437.bin");

lazy_static! {
    static ref TO_CP437: BTreeMap<char, u8> = {
        let mut m = BTreeMap::new();
        for (i, c) in std::str::from_utf8(UTF8_DATA).unwrap().chars().enumerate() {
            let result = m.insert(c, i as u8);
            assert!(result.is_none());
        }
        assert_eq!(m.len(), 256);
        m
    };
}

pub(crate) fn encode_lossy(c: char) -> u8 {
    if let Some(x) = TO_CP437.get(&c) { 
        return *x;
    }
    return b'?'
}