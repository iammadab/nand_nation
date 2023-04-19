use crate::bit::{Bit, Bit16};
use crate::chips::alu::adder::add16;
use crate::chips::and::and16;
use crate::chips::or::or16;

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
    todo!();
    // // first we preform the independent transformations on x and y
    // let x = apply_transformation(x, zx, nx);
    // let y = apply_transformation(y, zy, ny);
    //
    // // next we perform f
    // // if f is 0 we want to do and
    // // if f is 1 we want to do add
    // let (and_x, add_x) = dmux16(x, f);
    // let (and_y, add_y) = dmux16(y, f);
    //
    // let and_result = and16(and_x, and_y);
    // let add_result = add16(add_x, add_y);
    //
    // // compress the output
    // let output = or16(and_result, add_result);
    //
    // // apply transformation to the output
    // // zo is set to 0 by default as we never do that
    // let output = apply_transformation(output, 0, no);
    //
    // (output, Bit::Zero, Bit::Zero)
}
