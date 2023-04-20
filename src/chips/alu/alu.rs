use crate::bit::{Bit, Bit16};
use crate::chips::alu::adder::add16;
use crate::chips::and::{and16, and_bit16_with_bit};
use crate::chips::mux::mux16;
use crate::chips::not::{not, not16};
use crate::chips::or::{or16, or16way, or8way};

/// Alu Chip
/// Inputs
///     two 16 bit values x and y
///     6 control flags:
///         nx, ny, no -> negates x, y or o if set
///         zx, zy -> zeros x or y if set
///         f -> if set to 1 perform add(x, y) if set to 0 perform and(x, y)
/// Returns output o, zr and ng
///     zr - set if output bits are all zeros
///     ng - set if output is negative
fn alu(
    x: Bit16,
    y: Bit16,
    zx: Bit,
    nx: Bit,
    zy: Bit,
    ny: Bit,
    f: Bit,
    no: Bit,
) -> (Bit16, Bit, Bit) {
    // first we preform the independent transformations on x and y
    let x = apply_transformation(x, zx, nx);
    let y = apply_transformation(y, zy, ny);

    // perform f
    let and_result = and16(x, y);
    let add_result = add16(x, y);

    // compress the output
    let output = mux16(and_result, add_result, f);

    // apply transformation to the output
    // zo is set to 0 by default as we never do that
    let output = apply_transformation(output, Bit::Zero, no);
    let zr = check_all_zero(output);
    let ng = check_negative(output);

    (output, zr, ng)
}

fn apply_transformation(input: Bit16, zero_flag: Bit, negate_flag: Bit) -> Bit16 {
    // handle zero flag
    let zero_result = and_bit16_with_bit(input, Bit::Zero);
    let input = mux16(input, zero_result, zero_flag);

    // handle negation flag
    let negation_result = not16(input);
    let output = mux16(input, negation_result, negate_flag);

    output
}

fn check_all_zero(input: Bit16) -> Bit {
    not(or16way(input))
}

fn check_negative(input: Bit16) -> Bit {
    input[0]
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{alu_test, bit16string, bitstring};

    #[test]
    fn test_apply_transformation() {
        let input = bit16string!("0011110011000011");
        // set no flag, input should be output
        assert_eq!(apply_transformation(input, Bit::Zero, Bit::Zero), input);
        // perform only zero function
        assert_eq!(
            apply_transformation(input, Bit::One, Bit::Zero),
            Bit16::default()
        );
        // perform only negation
        assert_eq!(
            apply_transformation(input, Bit::Zero, Bit::One),
            bit16string!("1100001100111100")
        );
        // perform zero then negate should be all ones
        assert_eq!(
            apply_transformation(input, Bit::One, Bit::One),
            bit16string!("1111111111111111")
        );
    }

    #[test]
    fn alu_chip() {
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "1",
            "0",
            "1",
            "0",
            "1",
            "0",
            "0000000000000000",
            "1",
            "0"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "1",
            "1",
            "1",
            "1",
            "1",
            "1",
            "0000000000000001",
            "0",
            "0"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "1",
            "1",
            "1",
            "0",
            "1",
            "0",
            "1111111111111111",
            "0",
            "1"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "0",
            "0",
            "1",
            "1",
            "0",
            "0",
            "0000000000000000",
            "1",
            "0"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "1",
            "1",
            "0",
            "0",
            "0",
            "0",
            "1111111111111111",
            "0",
            "1"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "0",
            "0",
            "1",
            "1",
            "0",
            "1",
            "1111111111111111",
            "0",
            "1"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "1",
            "1",
            "0",
            "0",
            "0",
            "1",
            "0000000000000000",
            "1",
            "0"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "0",
            "0",
            "1",
            "1",
            "1",
            "1",
            "0000000000000000",
            "1",
            "0"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "1",
            "1",
            "0",
            "0",
            "1",
            "1",
            "0000000000000001",
            "0",
            "0"
        );
        alu_test!(
            "0000000000000000",
            "1111111111111111",
            "0",
            "1",
            "1",
            "1",
            "1",
            "1",
            "0000000000000001",
            "0",
            "0"
        );
        // alu_test!(
        //     "0000000000010001",
        //     "0000000000000011",
        //     "0",
        //     "1",
        //     "0",
        //     "1",
        //     "0",
        //     "1",
        //     "0000000000010011",
        //     "0",
        //     "0"
        // );
    }
}
