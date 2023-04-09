use std::ops::{Deref, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Bit {
    One,
    Zero,
}

#[derive(Debug, PartialEq)]
pub(crate) struct BitN<const N: usize>([Bit; N]);

impl<const N: usize> Default for BitN<N> {
    fn default() -> Self {
        BitN([Bit::Zero; N])
    }
}

impl<const N: usize> Index<usize> for BitN<N> {
    type Output = Bit;
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const N: usize> IndexMut<usize> for BitN<N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const N: usize> From<String> for BitN<N> {
    fn from(value: String) -> BitN<N> {
        debug_assert_eq!(value.len(), N);
        let mut result: BitN<N> = BitN::default();
        for (i, char) in value.chars().enumerate() {
            match char {
                '1' => result[i] = Bit::One,
                _ => result[i] = Bit::Zero,
            }
        }
        result
    }
}

impl<const N: usize> From<[Bit; N]> for BitN<N> {
    fn from(value: [Bit; N]) -> BitN<N> {
        BitN::<N>(value)
    }
}

pub(crate) type Bit16 = BitN<16>;

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::bit::{Bit, Bit16};

    #[test]
    fn bit16_from_string() {
        let bits = Bit16::from(String::from("0000000000000000"));
        assert_eq!(bits, Bit16::from([Zero; 16]));
        let bits = Bit16::from(String::from("1111111111111111"));
        assert_eq!(bits, Bit16::from([One; 16]));
        let bits = Bit16::from(String::from("1010101010101010"));
        assert_eq!(
            bits,
            Bit16::from([
                One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One,
                Zero
            ])
        );
    }
}
