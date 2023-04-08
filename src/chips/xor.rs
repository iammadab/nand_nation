use crate::bit::Bit;
use crate::chips::and::and;
use crate::chips::not::not;
use crate::chips::or::or;

pub(crate) fn xor(a: Bit, b: Bit) -> Bit {
    // xor = ab' + a'b
    let first_term = and(a, not(b));
    let second_term = and(not(a), b);
    or(first_term, second_term)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};

    #[test]
    fn xor_gate() {
        assert_eq!(xor(Zero, Zero), Zero);
        assert_eq!(xor(Zero, One), One);
        assert_eq!(xor(One, Zero), One);
        assert_eq!(xor(One, One), Zero);
    }
}
