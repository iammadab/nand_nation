use crate::bit::Bit;
use crate::chips::memory::dff::DFF;
use crate::chips::mux::mux;

/// 1 Bit Register
/// Input: in, load
/// Output: out
/// Function: if load(t-1) then out(t) = in(t - 1)
///     else out(t) = out(t - 1)
struct Register {
    dff: DFF,
}

impl Register {
    fn new() -> Self {
        Self { dff: DFF::new() }
    }

    fn clock(&mut self, input: Bit, load: Bit) -> Bit {
        // if the load bit is set, we feed forward the provided input
        // if not set we feed forward the last output of the dff
        let dff_input = mux(self.dff.stored_bit, input, load);
        self.dff.clock(dff_input)
    }
}

#[cfg(test)]
mod test {
    use crate::bit::Bit;
    use crate::chips::memory::register::Register;
    use crate::testing::TestReader;

    #[test]
    fn register_gate() {
        let mut register = Register::new();

        let test_tokens = TestReader::read("register.txt");
        let mut token_iter = test_tokens.into_iter();
        // skip the header
        let mut token_iter = token_iter.skip(4);

        // continue as long as we have clock input
        while token_iter.next().is_some() {
            let input = Bit::from(token_iter.next().unwrap());
            let load = Bit::from(token_iter.next().unwrap());
            let out = Bit::from(token_iter.next().unwrap());

            assert_eq!(register.clock(input, load), out);
        }
    }
}