use std::cmp::Ordering;

/// Converts an iterator of bools representing the bits of a number into that number
fn bool_iter_to_num(it: impl DoubleEndedIterator<Item = bool>) -> usize {
    it.map(|b| b as usize)
        .rev()
        .enumerate()
        .fold(0, |acc, (i, b)| acc + (b << i))
}

fn task1(lines: &[&str]) {
    let line_length = lines[0].len();

    // Convert each line into an iterator of bools, then flatten it so we get
    // one stream of bools
    let bits = lines.iter().flat_map(|&l| l.chars()).map(|c| c == '1');

    // This counts the number of 1s in each column and returns true if they are
    // more common than 0. This results in an iterator of bools that corresponds
    // to the bits in the gamma rate
    let most_common_bits = (0..line_length)
        .map(|i| {
            bits.clone()
                .skip(i)
                .step_by(line_length)
                .filter(|&b| b)
                .count()
        })
        .map(|num_ones| num_ones > (lines.len() - num_ones));

    let gamma_rate = bool_iter_to_num(most_common_bits.clone());
    // If we flip the bits of gamma_rate, we get epsilon rate
    let epsilon_rate = bool_iter_to_num(most_common_bits.map(|b| !b));

    println!(
        "Task 1: Gamma rate = {}, Epsilon rate = {}",
        gamma_rate, epsilon_rate
    );
    println!("Task 1 Answer: {}", gamma_rate * epsilon_rate);
}

// If most_common is true, look for the most common value, otherwise look for
// least common value
fn search_for_rating<'a>(search_space: &[&'a str], bit_idx: usize, most_common: bool) -> &'a str {
    match search_space.len().cmp(&1) {
        // Base case: we've found the line we want
        Ordering::Equal => search_space[0],
        // Alternative case: we need to reduce the search space
        Ordering::Greater => {
            // Find the index of the first 1 in the bit_idx column
            let one_idx = search_space
                .iter()
                .map(|l| l.chars().nth(bit_idx).unwrap())
                .position(|b| b == '1')
                // If there aren't any 1s, return the last index
                .unwrap_or(search_space.len() - 1);

            // Use the index of the first 1 in relation to the halfway point of
            // the search space to work out which number is most common. We then
            // reduce the search space to only the lines which have that number
            // in the bit_idx column.
            // The xor shenangians will flip the result of the condition when
            // most_common is false so we find the least common value instead
            let new_search_space = if (one_idx <= search_space.len() / 2) ^ !most_common {
                // 1s are the most/least common
                &search_space[one_idx..search_space.len()]
            } else {
                // 0s are the most/least common
                &search_space[0..one_idx]
            };

            // Search the next column using the remaining lines
            search_for_rating(new_search_space, bit_idx + 1, most_common)
        }
        Ordering::Less => unreachable!(),
    }
}

fn task2(lines: &mut [&str]) {
    // If we sort the lines lexicographically, we can recursively search for the
    // line we want
    lines.sort_unstable();

    let o2_line = search_for_rating(lines, 0, true);
    let co2_line = search_for_rating(lines, 0, false);

    let o2_rating = bool_iter_to_num(o2_line.chars().map(|c| c == '1'));
    let co2_rating = bool_iter_to_num(co2_line.chars().map(|c| c == '1'));

    println!(
        "Task 2: O2 generator rating = {}, CO2 scrubber rating = {}",
        o2_rating, co2_rating
    );
    println!("Task 2 Answer: {}", o2_rating * co2_rating);
}

fn main() {
    let mut lines: Vec<&str> = include_str!("input.txt").lines().collect();

    task1(&lines);
    task2(&mut lines);
}
