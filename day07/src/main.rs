fn task1_dist(crab_pos: i32, pos: i32) -> i32 {
    (crab_pos - pos).abs()
}

fn task2_dist(crab_pos: i32, pos: i32) -> i32 {
    let dist = (crab_pos - pos).abs();

    (dist * (dist + 1)) / 2
}

fn main() {
    let mut positions: Vec<i32> = include_str!("input.txt")
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    // The median of the positions is the optimal position
    positions.sort_unstable();
    let optimal_pos = positions[(positions.len() / 2) - 1];

    let fuel_task1: i32 = positions
        .iter()
        .map(|&crab_pos| task1_dist(crab_pos, optimal_pos))
        .sum();

    println!("Task 1: {}", fuel_task1);

    // I just brute forced Task 2 lol
    let &min = positions.iter().min().unwrap();
    let &max = positions.iter().max().unwrap();

    let fuel_task2: i32 = (min..max)
        .map(|pos| {
            positions
                .iter()
                .map(move |&crab_pos| task2_dist(crab_pos, pos))
                .sum()
        })
        .min()
        .unwrap();

    println!("Task 2: {}", fuel_task2);
}
