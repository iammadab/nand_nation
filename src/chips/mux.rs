use crate::bit::Bit;
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
}
