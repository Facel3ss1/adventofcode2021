fn task1_dist(crab_pos: i32, pos: i32) -> i32 {
    (crab_pos - pos).abs()
}

fn task2_dist(crab_pos: i32, pos: i32) -> i32 {
    let dist = (crab_pos - pos).abs();

    // Triangle numbers
    (dist * (dist + 1)) / 2
}

fn main() {
    let mut positions: Vec<i32> = include_str!("input.txt")
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    // The median of the positions is the optimal position.
    // This is because the median minimises the distances to all of the points.
    positions.sort_unstable();
    let median_pos = positions[(positions.len() / 2) - 1];

    let fuel_task1: i32 = positions
        .iter()
        .map(|&crab_pos| task1_dist(crab_pos, median_pos))
        .sum();

    println!("Task 1: {}", fuel_task1);

    // The mean minimises the squared distances to all of the points.
    // The distance to a point is (n^2 + n)/2, so we actually need to minimise
    // n^2 + n, but our data set is small enough that we can just find the mean
    // to minimise n^2 and check the floor and the ceiling.
    let mean_floor = positions.iter().sum::<i32>() / (positions.len() as i32);
    let mean_ceil = mean_floor + 1;

    let fuel_task2: i32 = [mean_floor, mean_ceil]
        .iter()
        .map(|&pos| {
            positions
                .iter()
                .map(|&crab_pos| task2_dist(crab_pos, pos))
                .sum()
        })
        .min()
        .unwrap();

    println!("Task 2: {}", fuel_task2);
}
