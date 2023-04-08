use crate::bit::Bit;
use crate::bit::Bit::{One, Zero};
use crate::chips::nand::nand;

pub(crate) fn not(a: Bit) -> Bit {
    nand(a, One)
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::chips::not::not;

    #[test]
    fn not_gate() {
        assert_eq!(not(One), Zero);
        assert_eq!(not(Zero), One);
    }
}
