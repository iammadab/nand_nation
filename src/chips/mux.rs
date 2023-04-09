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
    use crate::{bitstring, mux16test};

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
        mux16test!(
            "0000000000000000",
            "0000000000000000",
            Zero,
            "0000000000000000"
        );
        mux16test!(
            "0000000000000000",
            "0000000000000000",
            One,
            "0000000000000000"
        );
        mux16test!(
            "0000000000000000",
            "0001001000110100",
            Zero,
            "0000000000000000"
        );
        mux16test!(
            "0000000000000000",
            "0001001000110100",
            One,
            "0001001000110100"
        );
        mux16test!(
            "1001100001110110",
            "0000000000000000",
            Zero,
            "1001100001110110"
        );
        mux16test!(
            "1001100001110110",
            "0000000000000000",
            One,
            "0000000000000000"
        );
        mux16test!(
            "1010101010101010",
            "0101010101010101",
            Zero,
            "1010101010101010"
        );
        mux16test!(
            "1010101010101010",
            "0101010101010101",
            One,
            "0101010101010101"
        );
    }
}
