use std::ops::{Deref, DerefMut, Index, IndexMut};

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Bit {
    One,
    Zero,
}

impl From<String> for Bit {
    fn from(value: String) -> Self {
        value
            .chars()
            .next()
            .map(|ch| match ch {
                '1' => Bit::One,
                _ => Bit::Zero,
            })
            .unwrap_or(Bit::Zero)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(crate) struct BitN<const N: usize>(pub(crate) [Bit; N]);

impl<const N: usize> BitN<N> {
    pub(crate) fn truncate<const T: usize>(&self) -> [Bit; T] {
        debug_assert!(T < N);
        let mut output = [Bit::Zero; T];
        // take the last T bits from self
        for i in 0..T {
            output[i] = self.0[N - T + i];
        }
        output
    }
}

impl<const N: usize> Default for BitN<N> {
    fn default() -> Self {
        BitN([Bit::Zero; N])
    }
}

impl<const N: usize> Deref for BitN<N> {
    type Target = [Bit; N];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<const N: usize> DerefMut for BitN<N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
pub(crate) type Bit8 = BitN<8>;
pub(crate) type Bit2 = BitN<2>;
pub(crate) type Bit3 = BitN<3>;
pub(crate) type Bit4 = BitN<4>;

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::bit::{Bit, Bit16, Bit3, Bit4, Bit8};
    use crate::{bit3string, bit4string};

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

    #[test]
    #[should_panic]
    fn cannot_build_bit16_from_invalid_len_string() {
        let bits = Bit16::from(String::from("0"));
    }

    #[test]
    fn bit8_from_string() {
        let bits = Bit8::from(String::from("00000000"));
        assert_eq!(bits, Bit8::from([Zero; 8]));
        let bits = Bit8::from(String::from("11111111"));
        assert_eq!(bits, Bit8::from([One; 8]));
        let bits = Bit8::from(String::from("10101010"));
        assert_eq!(
            bits,
            Bit8::from([One, Zero, One, Zero, One, Zero, One, Zero,])
        );
    }

    #[test]
    fn bit_truncation() {
        let seven_16_bit = Bit16::from(String::from("0000000000000111"));
        let seven_3_bit: Bit3 = seven_16_bit.truncate::<3>().into();
        assert_eq!(seven_3_bit, bit3string!("111"));
        let seven_4_bit: Bit4 = seven_16_bit.truncate::<4>().into();
        assert_eq!(seven_4_bit, bit4string!("0111"));
    }
}
