use crate::bit::Bit;
use crate::bit::Bit::{One, Zero};

// Primitive gate, as such we implement the truth table directly
pub(crate) fn nand(a: Bit, b: Bit) -> Bit {
    match (a, b) {
        (Zero, Zero) => One,
        (Zero, One) => One,
        (One, Zero) => One,
        (One, One) => Zero,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn nand_gate() {
        assert_eq!(nand(Zero, Zero), One);
        assert_eq!(nand(Zero, One), One);
        assert_eq!(nand(One, Zero), One);
        assert_eq!(nand(One, One), Zero);
    }
}
