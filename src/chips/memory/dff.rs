use crate::bit::Bit;

// TODO: this can't be built from nand gates
//  the book considers this a primitive gate
//  but will be a good learning experience to
//  build this from elementary gates
/// DFF
/// Input: in (single bit)
/// Output: out (single bit)
/// Function: out(t) = in(t - 1)
///     at clock init out(t) = 0
#[derive(Clone, Copy)]
pub(crate) struct DFF {
    pub(crate) stored_bit: Bit,
}

impl DFF {
    pub(crate) fn new() -> Self {
        Self {
            stored_bit: Bit::Zero,
        }
    }

    pub(crate) fn clock(&mut self, input: Bit) -> Bit {
        let mut stored_bit = self.stored_bit;
        self.stored_bit = input;
        stored_bit
    }
}

#[cfg(test)]
mod test {
    use crate::bit::Bit;
    use crate::chips::memory::dff::DFF;

    #[test]
    fn dff_gate() {
        let mut dff = DFF::new();
        // out at first clock is 0
        assert_eq!(dff.clock(Bit::Zero), Bit::Zero);
        // check previous
        assert_eq!(dff.clock(Bit::One), Bit::Zero);
        assert_eq!(dff.clock(Bit::Zero), Bit::One);
        assert_eq!(dff.clock(Bit::Zero), Bit::Zero);
    }
}
