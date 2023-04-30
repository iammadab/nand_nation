use crate::bit::{Bit16, Bit3};
use crate::chips::alu::inc::inc16;
use crate::chips::not::not16;

pub(crate) fn two_complement16(input: Bit16) -> Bit16 {
    // negate all the bits
    let negated_input = not16(input);
    // add one to the result
    inc16(negated_input)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bit16string;
    use crate::chips::alu::two_complement::two_complement16;

    #[test]
    fn two_complement16_gate() {
        let one = bit16string!("0000000000000001");
        assert_eq!(two_complement16(one), bit16string!("1111111111111111"));

        let seven = bit16string!("0000000000000111");
        assert_eq!(two_complement16(seven), bit16string!("1111111111111001"));
    }
}
