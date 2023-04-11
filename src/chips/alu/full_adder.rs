use crate::bit::Bit;
use crate::chips::alu::half_adder::half_adder;

/// FullAdder
/// Inputs: a, b, c
/// Outputs: sum, carry
pub(crate) fn full_adder(a: Bit, b: Bit, c: Bit) -> (Bit, Bit) {
    let (s1, c1) = half_adder(a, b);
    let (final_sum, c2) = half_adder(s1, c);
    let (final_carry, _) = half_adder(c1, c2);
    (final_sum, final_carry)
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::chips::alu::full_adder::full_adder;

    #[test]
    fn full_adder_chip() {
        assert_eq!(full_adder(Zero, Zero, Zero), (Zero, Zero));
        assert_eq!(full_adder(Zero, Zero, One), (One, Zero));
        assert_eq!(full_adder(Zero, One, Zero), (One, Zero));
        assert_eq!(full_adder(Zero, One, One), (Zero, One));
        assert_eq!(full_adder(One, Zero, Zero), (One, Zero));
        assert_eq!(full_adder(One, Zero, One), (Zero, One));
        assert_eq!(full_adder(One, One, Zero), (Zero, One));
        assert_eq!(full_adder(One, One, One), (One, One));
    }
}
