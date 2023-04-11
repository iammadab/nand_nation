use crate::bit::Bit::Zero;
use crate::bit::{Bit, Bit16};
use crate::chips::alu::full_adder::full_adder;

pub(crate) fn add16(a: Bit16, b: Bit16) -> Bit16 {
    // default carry is 0
    let mut carry = Zero;
    let mut result = Bit16::default();
    // reversing cause we want to sum LSB first
    for i in (0..16).rev() {
        let (sum, new_carry) = full_adder(a[i], b[i], carry);
        result[i] = sum;
        carry = new_carry;
    }

    result
}

#[cfg(test)]
mod test {
    use crate::bit::Bit16;
    use crate::chips::alu::adder::add16;
    use crate::{add16test, bit16string};

    #[test]
    fn add16_chip() {
        add16test!("0000000000000000", "0000000000000000", "0000000000000000");
        add16test!("0000000000000000", "1111111111111111", "1111111111111111");
        add16test!("1111111111111111", "1111111111111111", "1111111111111110");
        add16test!("1010101010101010", "0101010101010101", "1111111111111111");
        add16test!("0011110011000011", "0000111111110000", "0100110010110011");
        add16test!("0001001000110100", "1001100001110110", "1010101010101010");
    }
}
