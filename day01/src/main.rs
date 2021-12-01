use std::num::ParseIntError;

use itertools::Itertools;

fn task1(input: impl Iterator<Item = i32>) -> i32 {
    input
        .tuple_windows()
        .fold(0, |inc, (a, b)| if b > a { inc + 1 } else { inc })
}

fn task2(input: impl Iterator<Item = i32>) -> i32 {
    input
        // This is the same but I added these two lines to sum up the windows
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |inc, (a, b)| if b > a { inc + 1 } else { inc })
}

fn main() -> Result<(), ParseIntError> {
    let input: Vec<i32> = include_str!("input.txt")
        .lines()
        .map(str::parse::<i32>)
        .try_collect()?;

    println!(
        "Task 1: {} measurements were larger than the previous measurement",
        task1(input.iter().copied())
    );

    println!(
        "Task 2: {} sums were larger than the previous sum",
        task2(input.iter().copied())
    );

    Ok(())
}
