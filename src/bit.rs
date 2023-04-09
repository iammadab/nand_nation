use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Bit {
    One,
    Zero,
}

#[derive(Debug, PartialEq)]
struct Bit16([Bit; 16]);

impl Default for Bit16 {
    fn default() -> Self {
        Bit16([Bit::Zero; 16])
    }
}

impl Index<usize> for Bit16 {
    type Output = Bit;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Bit16 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl From<String> for Bit16 {
    fn from(value: String) -> Bit16 {
        debug_assert_eq!(value.len(), 16);
        let mut result = Bit16::default();
        for (i, char) in value.chars().enumerate() {
            match char {
                '1' => result[i] = Bit::One,
                _ => result[i] = Bit::Zero,
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::bit::{Bit, Bit16};

    #[test]
    fn bit16_from_string() {
        let bits = Bit16::from(String::from("0000000000000000"));
        assert_eq!(bits, Bit16([Zero; 16]));
        let bits = Bit16::from(String::from("1111111111111111"));
        assert_eq!(bits, Bit16([One; 16]));
        let bits = Bit16::from(String::from("1010101010101010"));
        assert_eq!(
            bits,
            Bit16([
                One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One,
                Zero
            ])
        );
    }
}
