use crate::bit::Bit;
use crate::bit::Bit::{One, Zero};
use crate::chips::nand::nand;

pub(crate) fn not(a: Bit) -> Bit {
    nand(a, One)
}

pub(crate) fn not16(input: [Bit; 16]) -> [Bit; 16] {
    multi_bit_not(input)
}

fn multi_bit_not<const N: usize>(input: [Bit; N]) -> [Bit; N] {
    let mut result = [Bit::Zero; N];
    for i in 0..N {
        result[i] = not(input[i])
    }
    result
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::chips::not::{not, not16};

    #[test]
    fn not_gate() {
        assert_eq!(not(One), Zero);
        assert_eq!(not(Zero), One);
    }

    #[test]
    fn not16_gate() {
        assert_eq!(not16([Zero; 16]), [One; 16]);
        assert_eq!(not16([One; 16]), [Zero; 16]);
        assert_eq!(
            not16([
                One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One,
                Zero
            ]),
            [
                Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero, One, Zero,
                One
            ],
        )
    }
}
