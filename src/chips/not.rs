use crate::bit::Bit::{One, Zero};
use crate::bit::{Bit, Bit16};
use crate::chips::nand::nand;

pub(crate) fn not(a: Bit) -> Bit {
    nand(a, One)
}

pub(crate) fn not16(input: Bit16) -> Bit16 {
    let mut result = Bit16::default();
    for i in 0..16 {
        result[i] = not(input[i])
    }
    result
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::bit::Bit16;
    use crate::chips::not::{not, not16};
    use crate::{bitstring, not16test};

    #[test]
    fn not_gate() {
        assert_eq!(not(One), Zero);
        assert_eq!(not(Zero), One);
    }

    #[test]
    fn not16_gate() {
        not16test!("0000000000000000", "1111111111111111");
        not16test!("1111111111111111", "0000000000000000");
        not16test!("1010101010101010", "0101010101010101");
        not16test!("0011110011000011", "1100001100111100");
        not16test!("0001001000110100", "1110110111001011");
    }
}
