use crate::bit::Bit16;
use crate::bit16string;
use crate::chips::alu::adder::add16;

pub(crate) fn inc16(input: Bit16) -> Bit16 {
    let one_as_16_bit = bit16string!("0000000000000001");
    add16(input, one_as_16_bit)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit16string;
    use crate::chips::alu::inc::inc16;

    #[test]
    fn inc16_chip() {
        assert_eq!(
            inc16(bit16string!("0000000000000000")),
            bit16string!("0000000000000001")
        );
        assert_eq!(
            inc16(bit16string!("1111111111111111")),
            bit16string!("0000000000000000")
        );
        assert_eq!(
            inc16(bit16string!("0000000000000101")),
            bit16string!("0000000000000110")
        );
        assert_eq!(
            inc16(bit16string!("1111111111111011")),
            bit16string!("1111111111111100")
        );
    }
}
