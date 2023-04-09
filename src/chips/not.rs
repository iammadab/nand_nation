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

    #[test]
    fn not_gate() {
        assert_eq!(not(One), Zero);
        assert_eq!(not(Zero), One);
    }

    #[test]
    fn not16_gate() {
        assert_eq!(
            not16(Bit16::from(String::from("0000000000000000"))),
            Bit16::from(String::from("1111111111111111"))
        );
        assert_eq!(
            not16(Bit16::from(String::from("1111111111111111"))),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            not16(Bit16::from(String::from("1010101010101010"))),
            Bit16::from(String::from("0101010101010101"))
        );
        assert_eq!(
            not16(Bit16::from(String::from("0011110011000011"))),
            Bit16::from(String::from("1100001100111100"))
        );
        assert_eq!(
            not16(Bit16::from(String::from("0001001000110100"))),
            Bit16::from(String::from("1110110111001011"))
        );
    }
}
