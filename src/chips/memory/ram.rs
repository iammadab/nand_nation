use crate::bit::{Bit, Bit16, Bit3, Bit6, Bit9};
use crate::chips::dmux::dmux8way;
use crate::chips::memory::register::Register;
use crate::chips::mux::mux8way16;

/// RAM8
/// Input: in[16], address[3], load
/// Output: out[16]
/// Function: out(t) = RAM[address(t)](t) - what does register at the provided address contain
///     if load is set, then store the input in the register at current address
///     at next clock cycle we shall have access to this stored value
#[derive(Copy, Clone)]
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

/// RAM64
/// Input: in[16], address[6], load
/// Output: out[16]
/// Function: same as above
#[derive(Copy, Clone)]
struct RAM64 {
    memory: [RAM8; 8],
}

impl RAM64 {
    fn new() -> Self {
        Self {
            memory: [RAM8::new(); 8],
        }
    }

    pub(crate) fn clock(&mut self, input: Bit16, address: Bit6, load: Bit) -> Bit16 {
        // We use the first 3 bits of the address to select which RAM8 gets
        // the load value
        let ram_8_address: Bit3 = [address[0], address[1], address[2]].into();
        let register_address: Bit3 = [address[3], address[4], address[5]].into();

        let ram_8_load = dmux8way(load, ram_8_address);

        let mut ram_8_outputs = Vec::new();
        for i in 0..8 {
            ram_8_outputs.push(self.memory[i].clock(input, register_address, ram_8_load[i]));
        }

        mux8way16(ram_8_outputs.try_into().unwrap(), ram_8_address)
    }
}

/// RAM512
/// Input: in[16], address[9], load
/// Output: out[16]
/// Function: same as above
#[derive(Copy, Clone)]
struct RAM512 {
    memory: [RAM64; 8],
}

impl RAM512 {
    fn new() -> Self {
        Self {
            memory: [RAM64::new(); 8],
        }
    }

    pub(crate) fn clock(&mut self, input: Bit16, address: Bit9, load: Bit) -> Bit16 {
        let memory_address: Bit3 = [address[0], address[1], address[2]].into();
        let register_address: Bit6 = [
            address[3], address[4], address[5], address[6], address[7], address[8],
        ]
        .into();

        let load_values = dmux8way(load, memory_address);

        let mut memory_outputs = Vec::new();
        for i in 0..8 {
            memory_outputs.push(self.memory[i].clock(input, register_address, load_values[i]));
        }

        mux8way16(memory_outputs.try_into().unwrap(), memory_address)
    }
}

#[cfg(test)]
mod test {
    use crate::bit::{Bit, Bit16, Bit3, Bit6, Bit9};
    use crate::chips::memory::ram::{RAM512, RAM64, RAM8};
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

    #[test]
    fn ram64_gate() {
        let mut ram_64 = RAM64::new();

        let test_tokens = TestReader::read("ram64.txt");
        let mut token_iter = test_tokens.into_iter();
        // skip the header
        let mut token_iter = token_iter.skip(5);

        // continue as long as we have clock input
        while token_iter.next().is_some() {
            let input = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let load = Bit::from(token_iter.next().unwrap());
            let address = Bit6::from(TestReader::from_6_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let output = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            assert_eq!(ram_64.clock(input, address, load), output);
        }
    }

    #[test]
    fn ram512_gate() {
        let mut ram_512 = RAM512::new();

        let test_tokens = TestReader::read("ram512.txt");
        let mut token_iter = test_tokens.into_iter();
        // skip the header
        let mut token_iter = token_iter.skip(5);

        // continue as long as we have clock input
        while token_iter.next().is_some() {
            let input = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let load = Bit::from(token_iter.next().unwrap());
            let address = Bit9::from(TestReader::from_9_bit_int_string(
                token_iter.next().unwrap(),
            ));
            let output = Bit16::from(TestReader::from_16_bit_int_string(
                token_iter.next().unwrap(),
            ));
            assert_eq!(ram_512.clock(input, address, load), output);
        }
    }
}
