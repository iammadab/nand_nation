use crate::bit::{Bit, Bit2, Bit3, Bit4, Bit8};
use crate::chips::and::and;
use crate::chips::not::not;

/// Demultiplexor
/// Inputs: input, sel
/// Outputs: a, b
/// Function: if sel = 0 then { a = input, b = 0 } else { a = 0, b = input }
pub(crate) fn dmux(input: Bit, sel: Bit) -> (Bit, Bit) {
    // we have two outputs, so we need two boolean expressions
    // a = input . sel'
    // b = input . sel
    let a = and(input, not(sel));
    let b = and(input, sel);
    (a, b)
}

pub(crate) fn dmux4way(input: Bit, sel: Bit2) -> Bit4 {
    // Take a single bit and expand it into 4 output channels
    // putting the input in the position specified by the selector
    let (t1, t2) = dmux(input, sel[0]);
    let (a, b) = dmux(t1, sel[1]);
    let (c, d) = dmux(t2, sel[1]);
    Bit4::from([a, b, c, d])
}

pub(crate) fn dmux8way(input: Bit, sel: Bit3) -> Bit8 {
    let (t1, t2) = dmux(input, sel[0]);
    let m = dmux4way(t1, Bit2::from([sel[1], sel[2]]));
    let n = dmux4way(t2, Bit2::from([sel[1], sel[2]]));
    Bit8::from([m[0], m[1], m[2], m[3], n[0], n[1], n[2], n[3]])
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};
    use crate::{bit2string, bit3string, bit4string, bit8string};

    #[test]
    fn dmux_gate() {
        assert_eq!(dmux(Zero, Zero), (Zero, Zero));
        assert_eq!(dmux(Zero, One), (Zero, Zero));
        assert_eq!(dmux(One, Zero), (One, Zero));
        assert_eq!(dmux(One, One), (Zero, One));
    }

    #[test]
    fn dmux4way_gate() {
        assert_eq!(dmux4way(Zero, bit2string!("00")), bit4string!("0000"));
        assert_eq!(dmux4way(Zero, bit2string!("01")), bit4string!("0000"));
        assert_eq!(dmux4way(Zero, bit2string!("10")), bit4string!("0000"));
        assert_eq!(dmux4way(Zero, bit2string!("11")), bit4string!("0000"));
        assert_eq!(dmux4way(One, bit2string!("00")), bit4string!("1000"));
        assert_eq!(dmux4way(One, bit2string!("01")), bit4string!("0100"));
        assert_eq!(dmux4way(One, bit2string!("10")), bit4string!("0010"));
        assert_eq!(dmux4way(One, bit2string!("11")), bit4string!("0001"));
    }

    #[test]
    fn dmux8way_gate() {
        assert_eq!(dmux8way(Zero, bit3string!("000")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("001")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("010")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("011")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("100")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("101")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("110")), bit8string!("00000000"));
        assert_eq!(dmux8way(Zero, bit3string!("111")), bit8string!("00000000"));
        assert_eq!(dmux8way(One, bit3string!("000")), bit8string!("10000000"));
        assert_eq!(dmux8way(One, bit3string!("001")), bit8string!("01000000"));
        assert_eq!(dmux8way(One, bit3string!("010")), bit8string!("00100000"));
        assert_eq!(dmux8way(One, bit3string!("011")), bit8string!("00010000"));
        assert_eq!(dmux8way(One, bit3string!("100")), bit8string!("00001000"));
        assert_eq!(dmux8way(One, bit3string!("101")), bit8string!("00000100"));
        assert_eq!(dmux8way(One, bit3string!("110")), bit8string!("00000010"));
        assert_eq!(dmux8way(One, bit3string!("111")), bit8string!("00000001"));
    }
}
