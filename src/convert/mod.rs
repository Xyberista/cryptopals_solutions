type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub type Hex = String;

/// Bytes to hex
///
/// Values are padded with leading zero to two digits.
/// Letters are lowercase.
pub trait ToHex {
    fn to_hex(&self) -> Hex;
}

impl<I> ToHex for I
where
    I: IntoIterator<Item = u8> + Clone,
{
    fn to_hex(&self) -> Hex {
        self.clone()
            .into_iter()
            .flat_map(|d| [d >> 4, d & 0xf])
            .map(|u| char::from_digit(u32::from(u), 16).unwrap())
            .collect()
    }
}

#[allow(dead_code)]
pub type ByteVec = Vec<u8>;

pub trait ToByteVec {
    fn to_bytevec(&self) -> Result<ByteVec>;
}

impl<T> ToByteVec for T
where
    T: AsRef<str>,
{
    fn to_bytevec(&self) -> Result<Vec<u8>> {
        let origin = self.as_ref();
        if origin.len() % 2 != 0 {
            Err("Hex string not multiple of 2.".into())
        } else {
            origin
                .chars()
                .map(|c| c.to_digit(16))
                .collect::<Option<Vec<u32>>>()
                .ok_or("invalid digit".into())
                .map(|chars| {
                    chars
                        .chunks_exact(2)
                        .map(|pair| pair.iter().map(|&v| v as u8))
                        .map(|mut pair| (pair.next().unwrap() << 4) + pair.next().unwrap())
                        .collect::<Vec<u8>>()
                })
        }
    }
}

pub type Base64 = String;

pub trait ToBase64 {
    fn to_base64(&self) -> Base64;
}

impl<I> ToBase64 for I
where
    I: IntoIterator<Item = u8> + Clone,
{
    fn to_base64(&self) -> Base64 {
        let bytes = self.clone().into_iter().collect::<Vec<u8>>();
        let rem = if bytes.len() % 3 == 0 {
            0
        } else {
            3 - bytes.len() % 3
        };

        let mut sub = bytes
            .iter()
            .chain(vec![0; rem].iter())
            .copied()
            .collect::<Vec<u8>>()
            .chunks_exact(3)
            .map(|o| octets_to_base64(o.try_into().unwrap()))
            .collect::<String>();

        sub.truncate(sub.len() - rem);
        sub.extend(vec!['='; rem]);
        sub
    }
}

fn octets_to_base64(_octets @ [a, b, c]: [u8; 3]) -> String {
    String::from_iter(
        [
            (a >> 2),
            ((a & 0b11) << 4) + (b >> 4),
            ((b & 0b1111) << 2) + (c >> 6),
            (c & 0b11_1111),
        ]
        .map(sextet_to_char),
    )
}

fn sextet_to_char(sextet: u8) -> char {
    let val = match sextet {
        0..=25 => sextet + 65,
        26..=51 => sextet + 71,
        52..=61 => sextet - 4,
        62 => 43,
        63 => 47,
        _ => unreachable!(),
    };
    val as char
}

#[cfg(test)]
mod test_hex_and_base64 {
    use super::*;

    #[test]
    fn to_hex_works() {
        assert_eq!("", [].to_hex());
        assert_eq!("01", [1].to_hex());
        assert_eq!("0f", [15].to_hex());
        assert_eq!("ff", [255].to_hex());
    }

    #[test]
    fn from_hex_string_works() {
        assert_eq!(Vec::<u8>::new(), "".to_bytevec().unwrap());
        assert_eq!(Vec::<u8>::new(), String::new().to_bytevec().unwrap());
        assert_eq!(vec![15], "0f".to_bytevec().unwrap());
        assert_eq!(vec![255], "ff".to_bytevec().unwrap());
        assert!("f".to_bytevec().is_err());
    }

    #[test]
    fn sextet_to_char_works() {
        assert_eq!('A', sextet_to_char(0));
        assert_eq!('a', sextet_to_char(26));
        assert_eq!('0', sextet_to_char(52));
        assert_eq!('+', sextet_to_char(62));
        assert_eq!('/', sextet_to_char(63));
    }

    #[test]
    fn octets_to_base64_works() {
        assert_eq!(46, 0b01101110 & 0b111111);
        assert_eq!("TWFu", octets_to_base64([77, 97, 110]))
    }

    #[test]
    fn bytes_to_base64_works() {
        assert_eq!("TWFu", [77, 97, 110].to_base64());
        assert_eq!("TWE=", [77, 97].to_base64());
        assert_eq!("TQ==", [77].to_base64());
    }
}
