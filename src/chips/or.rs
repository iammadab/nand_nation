use crate::bit::{Bit, Bit16};
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};

    #[test]
    fn or_gate() {
        assert_eq!(or(Zero, Zero), Zero);
        assert_eq!(or(Zero, One), One);
        assert_eq!(or(One, Zero), One);
        assert_eq!(or(One, One), One);
    }

    #[test]
    fn or16_gate() {
        assert_eq!(
            or16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("0000000000000000"))
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            or16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("1111111111111111"))
            ),
            Bit16::from(String::from("1111111111111111"))
        );
        assert_eq!(
            or16(
                Bit16::from(String::from("1111111111111111")),
                Bit16::from(String::from("1111111111111111"))
            ),
            Bit16::from(String::from("1111111111111111"))
        );
        assert_eq!(
            or16(
                Bit16::from(String::from("1010101010101010")),
                Bit16::from(String::from("0101010101010101"))
            ),
            Bit16::from(String::from("1111111111111111"))
        );
        assert_eq!(
            or16(
                Bit16::from(String::from("0011110011000011")),
                Bit16::from(String::from("0000111111110000"))
            ),
            Bit16::from(String::from("0011111111110011"))
        );
        assert_eq!(
            or16(
                Bit16::from(String::from("0001001000110100")),
                Bit16::from(String::from("1001100001110110"))
            ),
            Bit16::from(String::from("1001101001110110"))
        );
    }
}
