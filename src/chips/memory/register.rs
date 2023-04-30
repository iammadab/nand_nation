use crate::bit::{Bit, Bit16};
use crate::chips::memory::binary_cell::BinaryCell;

/// Register
/// Input: in[16], load
/// Output: out[16],
/// Function: if load(t-1) then out(t) = in(t-1)
///     else out(t) = out(t - 1)
pub(crate) struct Register {
    binary_cells: [BinaryCell; 16],
}

impl Register {
    pub(crate) fn new() -> Self {
        Register {
            binary_cells: [BinaryCell::new(); 16],
        }
    }

    pub(crate) fn clock(&mut self, input: Bit16, load: Bit) -> Bit16 {
        let mut result = Bit16::default();
        for i in 0..16 {
            result[i] = self.binary_cells[i].clock(input[i], load);
        }
        result
    }
}

#[cfg(test)]
mod tests {
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
            let input = TestReader::from_16_bit_int_string(token_iter.next().unwrap());
            let load = Bit::from(token_iter.next().unwrap());
            let out = TestReader::from_16_bit_int_string(token_iter.next().unwrap());
            assert_eq!(register.clock(input, load), out);
        }
    }
}
