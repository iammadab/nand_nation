use crate::bit::Bit;
use crate::bit::Bit::Zero;
use crate::chips::nand::nand;
use crate::chips::not::not;

pub(crate) fn and(a: Bit, b: Bit) -> Bit {
    // we just invert the nand result
    not(nand(a, b))
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::chips::and::and;

    #[test]
    fn and_gate() {
        assert_eq!(and(Zero, Zero), Zero);
        assert_eq!(and(Zero, One), Zero);
        assert_eq!(and(One, Zero), Zero);
        assert_eq!(and(One, One), One);
    }
}
