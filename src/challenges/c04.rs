use crate::{convert::ToByteVec, xor::Xor};

use super::c03::{decode_single_byte_xor, get_score};

pub fn run() {
    let contents = std::fs::read_to_string("./data/c04.txt").unwrap();
    let lines = contents
        .lines()
        .map(|s| s.to_bytevec().unwrap())
        .collect::<Vec<_>>();
    let out = lines
        .iter()
        .flat_map(|v| (0_8..=255_u8).map(|x| v.xor(&vec![x; v.len()])))
        .min_by_key(|k| get_score(k))
        .unwrap();
    println!("{}", String::from_utf8(out).unwrap());
}
