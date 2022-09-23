#![allow(clippy::cast_precision_loss)]
#![allow(clippy::naive_bytecount)]
#![allow(clippy::cast_sign_loss)] // possible casts are after applying a square function
#![allow(clippy::cast_possible_truncation)] // not possible since rounded before casting
use crate::{convert::ToByteVec, xor::Xor};

pub const BYTE_FREQUENCIES: [(u8, f32); 26] = [
    (b'e', 11.16),
    (b'a', 8.50),
    (b'r', 7.58),
    (b'i', 7.54),
    (b'o', 7.16),
    (b't', 6.95),
    (b'n', 6.65),
    (b's', 5.74),
    (b'l', 5.49),
    (b'c', 4.54),
    (b'u', 3.63),
    (b'd', 3.38),
    (b'p', 3.17),
    (b'm', 3.01),
    (b'h', 3.00),
    (b'g', 2.47),
    (b'b', 2.07),
    (b'f', 1.81),
    (b'y', 1.78),
    (b'w', 1.29),
    (b'k', 1.10),
    (b'v', 1.01),
    (b'x', 0.29),
    (b'z', 0.27),
    (b'j', 0.20),
    (b'q', 0.20),
];

pub fn decode_single_byte_xor(hex_str: &str) -> Vec<u8> {
    (0_u8..=255_u8)
        .map(|byte| {
            let decoded = hex_str
                .to_bytevec()
                .unwrap()
                .xor(&vec![byte; hex_str.len()]);
            let score = get_score(&decoded);
            (decoded, score)
        })
        .min_by_key(|(_message, score)| *score)
        .unwrap()
        .0
}

pub fn get_score(decoded: &[u8]) -> usize {
    f32::round(
        BYTE_FREQUENCIES
            .iter()
            .fold(0.0, |score, (byte, expected)| {
                score + (expected - get_frequency(*byte, decoded)).powi(2)
            }),
    ) as usize
}

fn get_frequency(b: u8, s: &[u8]) -> f32 {
    (s.iter().filter(|&&byte| byte == b).count() as f32) / (s.len() as f32) * 100.0
}

#[cfg(test)]
mod test_c03 {
    use super::{decode_single_byte_xor, get_frequency};

    #[test]
    fn get_frequency_works() {
        assert_eq!(0.0, get_frequency(b'a', &[0, 0, 0, 0]));
        assert_eq!(100.0, get_frequency(b'a', &[b'a'; 4]));
        assert_eq!(50.0, get_frequency(b'a', &[b'a', b'a', b'0', b'0']));
    }

    #[test]
    fn c03_works() {
        assert_eq!(
            "Cooking MC's like a pound of bacon",
            String::from_utf8(decode_single_byte_xor(
                "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736"
            )).unwrap()
        );
    }
}
