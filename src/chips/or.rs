use crate::bit::Bit;
use crate::chips::and::and;
use crate::chips::not::not;

pub(crate) fn or(a: Bit, b: Bit) -> Bit {
    // The or gate = a + b
    // this is sum of product expression which includes an or
    // so we have to convert it to a product of sum expression
    // to get rid of the or.
    // to do this we apply de morgan's theorem
    // resulting in a'.b' where ' represents not
    // then we negate the entire expression
    // or gate = (a'.b')'
    not(and(not(a), not(b)))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit::Bit::{One, Zero};

    #[test]
    fn or_gate() {
        assert_eq!(or(Zero, Zero), Zero);
        assert_eq!(or(Zero, One), One);
        assert_eq!(or(One, Zero), One);
        assert_eq!(or(One, One), One);
    }
}
