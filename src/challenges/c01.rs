use crate::convert::*;

pub fn hex_to_base64(hex: &str) -> String {
    hex.to_bytevec().unwrap().to_base64()
}

#[cfg(test)]
mod test_c01 {
    use super::*;

    #[test]
    fn hex_to_base64_works() {
        assert_eq!(
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t",
            hex_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d")
        )
    }
}
