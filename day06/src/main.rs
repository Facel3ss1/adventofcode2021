fn main() {
    let ages = include_str!("input.txt")
        .split(',')
        .map(str::parse::<usize>)
        .map(Result::unwrap);

    let mut population: [u64; 9] = [0; 9];

    for age in ages {
        population[age] += 1;
    }

    for _ in 0..80 {
        population.rotate_left(1);
        // Every time a new lanternfish is created, its parent resets to 6
        population[6] += population[8];
    }

    println!("Task 1: {}", population.iter().sum::<u64>());

    for _ in 0..(256 - 80) {
        population.rotate_left(1);
        population[6] += population[8];
    }

    println!("Task 2: {}", population.iter().sum::<u64>());
}
