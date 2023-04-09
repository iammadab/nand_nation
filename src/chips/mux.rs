use crate::bit::{Bit, Bit16};
use crate::chips::and::and;
use crate::chips::not::not;
use crate::chips::or::or;

pub(crate) fn mux(a: Bit, b: Bit, sel: Bit) -> Bit {
    // let c = sel
    // mux = ac' + bc
    let t1 = and(a, not(sel));
    let t2 = and(b, sel);
    or(t1, t2)
}

pub(crate) fn mux16(a: Bit16, b: Bit16, sel: Bit) -> Bit16 {
    let mut result = Bit16::default();
    for i in 0..16 {
        result[i] = mux(a[i], b[i], sel)
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};

    #[test]
    fn mux_gate() {
        assert_eq!(mux(Zero, Zero, Zero), Zero);
        assert_eq!(mux(Zero, One, Zero), Zero);
        assert_eq!(mux(One, Zero, Zero), One);
        assert_eq!(mux(One, One, Zero), One);
        assert_eq!(mux(Zero, Zero, One), Zero);
        assert_eq!(mux(Zero, One, One), One);
        assert_eq!(mux(One, Zero, One), Zero);
        assert_eq!(mux(One, One, One), One);
    }

    #[test]
    fn mux16_gate() {
        assert_eq!(
            mux16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("0000000000000000")),
                Zero
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("0000000000000000")),
                One
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("0001001000110100")),
                Zero
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("0000000000000000")),
                Bit16::from(String::from("0001001000110100")),
                One
            ),
            Bit16::from(String::from("0001001000110100"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("1001100001110110")),
                Bit16::from(String::from("0000000000000000")),
                Zero
            ),
            Bit16::from(String::from("1001100001110110"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("1001100001110110")),
                Bit16::from(String::from("0000000000000000")),
                One
            ),
            Bit16::from(String::from("0000000000000000"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("1010101010101010")),
                Bit16::from(String::from("0101010101010101")),
                Zero
            ),
            Bit16::from(String::from("1010101010101010"))
        );
        assert_eq!(
            mux16(
                Bit16::from(String::from("1010101010101010")),
                Bit16::from(String::from("0101010101010101")),
                One
            ),
            Bit16::from(String::from("0101010101010101"))
        );
    }
}
