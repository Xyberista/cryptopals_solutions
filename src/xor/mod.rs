pub trait Xor {
    fn xor(&self, other: &Self) -> Vec<u8>;
}

impl Xor for [u8] {
    fn xor(&self, other: &Self) -> Vec<u8> {
        self.iter().zip(other.iter()).map(|(a, b)| a ^ b).collect()
    }
}
