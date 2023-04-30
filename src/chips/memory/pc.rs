use crate::bit::{Bit, Bit16};
use crate::chips::alu::inc::inc16;
use crate::chips::memory::register::Register;
use crate::chips::mux::mux16;

/// Program counter
/// Input: in[16], inc, load, reset
/// Output: out[16]
/// Function: if reset(t-1) then out(t) = 0
///             else if load(t-1) then out(t)=in(t-1)
///                 else if inc(t-1) then out(t)=out(t-1)+1
///                     else out(t)=out(t-1)
struct PC {
    register: Register,
}

impl PC {
    fn new() -> Self {
        Self {
            register: Register::new(),
        }
    }

    fn clock(&mut self, input: Bit16, inc: Bit, load: Bit, reset: Bit) -> Bit16 {
        // setting load to 0 to force read
        let stored_value = self.register.clock(input, Bit::Zero);

        // inc step
        // we either increment the stored value or we don't
        let after_inc = mux16(stored_value, inc16(stored_value), inc);

        // load step
        // we need choose between the potentially incremented value or the input
        // load supersedes inc
        let after_load = mux16(after_inc, input, load);

        // reset step
        // we need to choose between the potential load value or the default value
        // reset supersedes load
        let after_reset = mux16(after_load, Bit16::default(), reset);

        // we always load the counter
        // the question is just what do we want to load
        self.register.clock(after_reset, Bit::One)
    }
}

#[cfg(test)]
mod test {
    use crate::bit::{Bit, Bit16};
    use crate::chips::memory::pc::PC;
    use crate::testing::TestReader;

    #[test]
    fn pc_gate() {
        let mut program_counter = PC::new();

        let test_tokens = TestReader::read("pc.txt");
        let mut token_iter = test_tokens.into_iter();
        // skip the header
        let mut token_iter = token_iter.skip(6);

        // continue as long as we have clock input
        while token_iter.next().is_some() {
            let input = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let reset = Bit::from(token_iter.next().unwrap());
            let load = Bit::from(token_iter.next().unwrap());
            let inc = Bit::from(token_iter.next().unwrap());
            let output = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            // dbg!(reset, load, inc);
            assert_eq!(program_counter.clock(input, inc, load, reset), output);
        }
    }
}
