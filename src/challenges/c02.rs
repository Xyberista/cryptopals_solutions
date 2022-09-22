use crate::convert::*;
use crate::xor::Xor;

pub fn fixed_xor(hex_a: &str, hex_b: &str) -> String {
    hex_a
        .to_bytevec()
        .unwrap()
        .xor(&hex_b.to_bytevec().unwrap())
        .to_hex()
}

#[cfg(test)]
mod test_fixed_xor {
    use super::fixed_xor;

    #[test]
    fn fixed_xor_works() {
        assert_eq!(
            "746865206b696420646f6e277420706c6179",
            fixed_xor(
                "1c0111001f010100061a024b53535009181c",
                "686974207468652062756c6c277320657965"
            )
        )
    }
}
