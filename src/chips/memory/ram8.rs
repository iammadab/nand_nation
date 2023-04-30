use crate::bit::{Bit, Bit16, Bit3};
use crate::chips::dmux::dmux8way;
use crate::chips::memory::register::Register;
use crate::chips::mux::mux8way16;

/// RAM8
/// Input: in[16], address[3], load
/// Output: out[16]
/// Function: out(t) = RAM[address(t)](t) - what does register at the provided address contain
///     if load is set, then store the input in the register at current address
///     at next clock cycle we shall have access to this stored value
struct RAM8 {
    registers: [Register; 8],
}

impl RAM8 {
    fn new() -> Self {
        Self {
            registers: [Register::new(); 8],
        }
    }

    pub(crate) fn clock(&mut self, input: Bit16, address: Bit3, load: Bit) -> Bit16 {
        // We only want to load a single register based on the address value
        // we use the dmux to set the load to 0 for registers we don't care about
        let load_values = dmux8way(load, address);

        // pass the input and the load bit to all registers
        // if load bit is set then only register will be set
        // aggregate the output from all registers
        let mut register_outputs = Vec::new();
        for i in 0..8 {
            register_outputs.push(self.registers[i].clock(input, load_values[i]))
        }

        // select the output for the register at the address we care about
        mux8way16(register_outputs.try_into().unwrap(), address)
    }
}

#[cfg(test)]
mod test {
    use crate::bit::{Bit, Bit16, Bit3};
    use crate::chips::memory::ram8::RAM8;
    use crate::testing::TestReader;

    #[test]
    fn ram8_gate() {
        let mut ram_8 = RAM8::new();

        let test_tokens = TestReader::read("ram8.txt");
        let mut token_iter = test_tokens.into_iter();
        // skip the header
        let mut token_iter = token_iter.skip(5);

        // continue as long as we have clock input
        while token_iter.next().is_some() {
            let input = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let load = Bit::from(token_iter.next().unwrap());
            let address = Bit3::from(TestReader::from_3_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let output = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            assert_eq!(ram_8.clock(input, address, load), output);
        }
    }
}
