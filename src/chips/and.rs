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

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::bit::{Bit, Bit16};
    use crate::chips::and::{and, and16};

    #[test]
    fn and_gate() {
        assert_eq!(and(Zero, Zero), Zero);
        assert_eq!(and(Zero, One), Zero);
        assert_eq!(and(One, Zero), Zero);
        assert_eq!(and(One, One), One);
    }

    #[test]
    fn and16_gate() {
        assert_eq!(
            and16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("0000000000000000"))
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            and16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("1111111111111111"))
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            and16(
                Bit16::from(String::from("1111111111111111")),
                Bit16::from(String::from("1111111111111111"))
            ),
            Bit16::from(String::from("1111111111111111"))
        );
        assert_eq!(
            and16(
                Bit16::from(String::from("1010101010101010")),
                Bit16::from(String::from("0101010101010101"))
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            and16(
                Bit16::from(String::from("0011110011000011")),
                Bit16::from(String::from("0000111111110000"))
            ),
            Bit16::from(String::from("0000110011000000"))
        );
        assert_eq!(
            and16(
                Bit16::from(String::from("0001001000110100")),
                Bit16::from(String::from("1001100001110110"))
            ),
            Bit16::from(String::from("0001000000110100"))
        );
    }
}
