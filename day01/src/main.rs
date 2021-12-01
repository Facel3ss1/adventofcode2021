use itertools::Itertools;

fn task1() {
    let num_increases = include_str!("input.txt")
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        .tuple_windows()
        .fold(0, |inc, (a, b)| if b > a { inc + 1 } else { inc });

    println!(
        "Task 1: {} measurements were larger than the previous measurement",
        num_increases
    );
}

fn task2() {
    let num_increases = include_str!("input.txt")
        .lines()
        .map(str::parse::<i32>)
        .map(Result::unwrap)
        // This is the same but I added these two lines to sum up the windows
        .tuple_windows()
        .map(|(a, b, c)| a + b + c)
        .tuple_windows()
        .fold(0, |inc, (a, b)| if b > a { inc + 1 } else { inc });

    println!(
        "Task 2: {} sums were larger than the previous sum",
        num_increases
    );
}

fn main() {
    task1();
    task2();
}
