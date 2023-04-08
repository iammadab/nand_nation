use crate::bit::Bit;
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};

    #[test]
    fn dmux_gate() {
        assert_eq!(dmux(Zero, Zero), (Zero, Zero));
        assert_eq!(dmux(Zero, One), (Zero, Zero));
        assert_eq!(dmux(One, Zero), (One, Zero));
        assert_eq!(dmux(One, One), (Zero, One));
    }
}
