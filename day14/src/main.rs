use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug)]
struct PairCounts {
    pair_counts: HashMap<(char, char), u64>,
    counts: HashMap<char, u64>,
    rules: HashMap<(char, char), char>,
}

impl PairCounts {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        let mut lines = lines.filter(|l| !l.is_empty());
        let first_line = lines.next().unwrap();

        let mut pair_counts = HashMap::new();

        for (a, b) in first_line.chars().tuple_windows() {
            *pair_counts.entry((a, b)).or_default() += 1;
        }

        let mut counts = HashMap::new();

        for char in first_line.chars() {
            *counts.entry(char).or_default() += 1;
        }

        let rules = lines
            .map(|line| {
                let mut rule = line.split(" -> ");
                let lhs = rule.next().unwrap();
                let rhs = rule.next().unwrap();

                (
                    lhs.chars().tuple_windows().next().unwrap(),
                    rhs.chars().next().unwrap(),
                )
            })
            .collect();

        Self {
            pair_counts,
            counts,
            rules,
        }
    }

    fn step(&mut self) {
        let mut new_pair_counts = HashMap::new();

        for (&pair, &count) in self.pair_counts.iter() {
            let (a, b) = pair;
            let &new_char = self.rules.get(&(a, b)).unwrap();

            *new_pair_counts.entry((a, new_char)).or_default() += count;
            *new_pair_counts.entry((new_char, b)).or_default() += count;

            *self.counts.entry(new_char).or_default() += count;
        }

        self.pair_counts = new_pair_counts;
    }

    fn answer(&self) -> u64 {
        let &max = self.counts.values().max().unwrap();
        let &min = self.counts.values().min().unwrap();

        max - min
    }
}

fn main() {
    let mut pair_counts = PairCounts::parse(include_str!("input.txt").lines());

    for _ in 0..10 {
        pair_counts.step();
    }

    // println!("{:#?}", pair_counts);

    println!("Task 1: {}", pair_counts.answer());

    for _ in 0..(40 - 10) {
        pair_counts.step();
    }

    println!("Task 2: {}", pair_counts.answer());
}
