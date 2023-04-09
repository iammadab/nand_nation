use crate::bit::{Bit, Bit16, Bit8};
use crate::chips::and::and;
use crate::chips::not::not;

pub(crate) fn or(a: Bit, b: Bit) -> Bit {
    // The or gate = a + b
    // this is sum of product expression which includes an or
    // so we have to convert it to a product of sum expression
    // to get rid of the or.
    // to do this we apply de morgan's theorem
    // resulting in a'.b' where ' represents not
    // then we negate the entire expression
    // or gate = (a'.b')'
    not(and(not(a), not(b)))
}

pub(crate) fn or16(a: Bit16, b: Bit16) -> Bit16 {
    let mut result = Bit16::default();
    for i in 0..16 {
        result[i] = or(a[i], b[i])
    }
    result
}

/// Or8Way
/// Input: Bit8
/// Output: Bit
/// Function: out = or(Bit8[0], Bit8[1], ... Bit8[7])
///     apply the or function to all the input bits simultaneously such that if
///     any bit is a 1 then the output is a 1
pub(crate) fn or8way(input: Bit8) -> Bit {
    let mut result = input[0];
    for i in 1..8 {
        result = or(result, input[i])
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};
    use crate::{bit16string, bit8string, or16test};

    #[test]
    fn or_gate() {
        assert_eq!(or(Zero, Zero), Zero);
        assert_eq!(or(Zero, One), One);
        assert_eq!(or(One, Zero), One);
        assert_eq!(or(One, One), One);
    }

    #[test]
    fn or16_gate() {
        or16test!("0000000000000000", "0000000000000000", "0000000000000000");
        or16test!("0000000000000000", "1111111111111111", "1111111111111111");
        or16test!("1111111111111111", "1111111111111111", "1111111111111111");
        or16test!("1010101010101010", "1010101010101010", "1010101010101010");
        or16test!("0011110011000011", "0000111111110000", "0011111111110011");
        or16test!("0001001000110100", "1001100001110110", "1001101001110110");
    }

    #[test]
    fn or8way_gate() {
        assert_eq!(or8way(bit8string!("00000000")), Zero);
        assert_eq!(or8way(bit8string!("11111111")), One);
        assert_eq!(or8way(bit8string!("00010000")), One);
        assert_eq!(or8way(bit8string!("00000001")), One);
        assert_eq!(or8way(bit8string!("00100110")), One);
    }
}
