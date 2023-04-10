use crate::bit::{Bit, Bit16, Bit2};
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

pub(crate) fn mux4way16(inputs: [Bit16; 4], sel: Bit2) -> Bit16 {
    // selector split the inputs into half recursively until we have only one output
    // 4 inputs -> 2 inputs -> 1 output
    // if a b c d are the inputs
    // the first selector chooses either {a, b} or {c, d}
    // hence we have to create multiplexer gates that interleave elements in the group
    // mux1(a, c) and mux2(b, d)
    // this way if selector is 0 then output will be a, b
    // and if 1 output will be c, d
    // use the output as new input and repeat.
    let t1 = mux16(inputs[0], inputs[2], sel[0]);
    let t2 = mux16(inputs[1], inputs[3], sel[0]);

    mux16(t1, t2, sel[1])
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};
    use crate::{bit16string, bit2string, mux16test, mux4way16test};

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

    #[test]
    fn mux4way16_gate() {
        mux4way16test!(
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "00",
            "0000000000000000"
        );
        mux4way16test!(
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "01",
            "0000000000000000"
        );
        mux4way16test!(
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "10",
            "0000000000000000"
        );
        mux4way16test!(
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "0000000000000000",
            "11",
            "0000000000000000"
        );
        mux4way16test!(
            "0001001000110100",
            "1001100001110110",
            "1010101010101010",
            "0101010101010101",
            "00",
            "0001001000110100"
        );
        mux4way16test!(
            "0001001000110100",
            "1001100001110110",
            "1010101010101010",
            "0101010101010101",
            "01",
            "1001100001110110"
        );
        mux4way16test!(
            "0001001000110100",
            "1001100001110110",
            "1010101010101010",
            "0101010101010101",
            "10",
            "1010101010101010"
        );
        mux4way16test!(
            "0001001000110100",
            "1001100001110110",
            "1010101010101010",
            "0101010101010101",
            "11",
            "0101010101010101"
        );
    }
}
