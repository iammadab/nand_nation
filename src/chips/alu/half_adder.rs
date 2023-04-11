use crate::bit::Bit;
use crate::chips::and::and;
use crate::chips::xor::xor;

/// HalfAdder
/// Inputs: a, b
/// Outputs: sum, carry
pub(crate) fn half_adder(a: Bit, b: Bit) -> (Bit, Bit) {
    let sum = xor(a, b);
    let carry = and(a, b);
    return (sum, carry);
}

#[cfg(test)]
mod test {
    use crate::bit::Bit::{One, Zero};
    use crate::chips::alu::half_adder::half_adder;

    #[test]
    fn half_adder_chip() {
        assert_eq!(half_adder(Zero, Zero), (Zero, Zero));
        assert_eq!(half_adder(Zero, One), (One, Zero));
        assert_eq!(half_adder(One, Zero), (One, Zero));
        assert_eq!(half_adder(One, One), (Zero, One));
    }
}
