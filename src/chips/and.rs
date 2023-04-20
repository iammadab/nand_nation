use crate::bit::Bit::Zero;
use crate::bit::{Bit, Bit16};
use crate::chips::nand::nand;
use crate::chips::not::not;

pub(crate) fn and(a: Bit, b: Bit) -> Bit {
    // we just invert the nand result
    not(nand(a, b))
}

pub(crate) fn and16(a: Bit16, b: Bit16) -> Bit16 {
    let mut result = Bit16::default();
    for i in 0..16 {
        result[i] = and(a[i], b[i])
    }
    result
}

/// Performs and operation on a 16 bit input
/// with a single bit value
pub(crate) fn and_bit16_with_bit(input: Bit16, b: Bit) -> Bit16 {
    let mut result = Bit16::default();
    for i in 0..16 {
        result[i] = and(input[i], b)
    }
    result
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::bit::{Bit, Bit16};
    use crate::chips::and::{and, and16, and_bit16_with_bit};
    use crate::{and16test, bit16string};

    #[test]
    fn and_gate() {
        assert_eq!(and(Zero, Zero), Zero);
        assert_eq!(and(Zero, One), Zero);
        assert_eq!(and(One, Zero), Zero);
        assert_eq!(and(One, One), One);
    }

    #[test]
    fn and16_gate() {
        and16test!("0000000000000000", "0000000000000000", "0000000000000000");
        and16test!("0000000000000000", "1111111111111111", "0000000000000000");
        and16test!("1111111111111111", "1111111111111111", "1111111111111111");
        and16test!("1010101010101010", "0101010101010101", "0000000000000000");
        and16test!("0011110011000011", "0000111111110000", "0000110011000000");
        and16test!("0001001000110100", "1001100001110110", "0001000000110100");
    }

    #[test]
    fn and_bit16_with_bit_gate() {
        // Sets all to zero if we perform with 0 bit
        assert_eq!(
            and_bit16_with_bit(bit16string!("1110001110001110"), Zero),
            bit16string!("0000000000000000")
        );
        // Returns the original input if we perform with 1
        assert_eq!(
            and_bit16_with_bit(bit16string!("1110001110001110"), One),
            bit16string!("1110001110001110")
        );
    }
}
