use snafu::prelude::*;

type SnafuResult<T, E = snafu::Whatever> = std::result::Result<T, E>;

fn subtract_numbers(a: u32, b: u32) -> SnafuResult<u32> {
    if a > b {
        Ok(a - b)
    } else {
        whatever!("Can't subtract {} - {}", a, b)
    }
}

fn complicated_math(a: u32, b: u32) -> SnafuResult<u32> {
    let val = subtract_numbers(a, b).whatever_context("Can't do the math")?;
    Ok(val * 2)
}

pub fn main() {
    let result = complicated_math(1, 2);
    println!("{:?}", result);
}
