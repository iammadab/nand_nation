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
    #[test]
    fn register_gate() {}
}
