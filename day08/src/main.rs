use bitflags::bitflags;
use itertools::Itertools;

fn task1(input: &[&str]) -> usize {
    input
        .iter()
        .flat_map(|line| line.split(" | ").nth(1).unwrap().split(' '))
        .filter(|output| {
            output.len() == 2 || output.len() == 4 || output.len() == 3 || output.len() == 7
        })
        .count()
}

bitflags! {
    struct Segment: u8 {
        const A = 0b0000001;
        const B = 0b0000010;
        const C = 0b0000100;
        const D = 0b0001000;
        const E = 0b0010000;
        const F = 0b0100000;
        const G = 0b1000000;
    }
}

const ZERO: Segment = Segment::all().difference(Segment::D);
const ONE: Segment = Segment::C.union(Segment::F);
const TWO: Segment = Segment::all().difference(Segment::B).difference(Segment::F);
const THREE: Segment = Segment::all().difference(Segment::B).difference(Segment::E);
const FOUR: Segment = Segment::B
    .union(Segment::C)
    .union(Segment::D)
    .union(Segment::F);
const FIVE: Segment = Segment::all().difference(Segment::C).difference(Segment::E);
const SIX: Segment = Segment::all().difference(Segment::C);
const SEVEN: Segment = Segment::A.union(Segment::C).union(Segment::F);
const EIGHT: Segment = Segment::all();
const NINE: Segment = Segment::all().difference(Segment::E);

impl Segment {
    fn parse(input: &str) -> Self {
        input
            .chars()
            .map(|c| match c {
                'a' => Self::A,
                'b' => Self::B,
                'c' => Self::C,
                'd' => Self::D,
                'e' => Self::E,
                'f' => Self::F,
                'g' => Self::G,
                _ => panic!("Unexpected character {}", c),
            })
            .fold(Self::empty(), |acc, seg| acc | seg)
    }

    /// Map a pattern to another pattern using the permutation
    fn map_permutation(&self, permutation: &[Self; 7]) -> Self {
        let mut output = Segment::empty();

        for (i, &perm_seg) in permutation.iter().enumerate() {
            if self.contains(perm_seg) {
                output |= match i {
                    0 => Self::A,
                    1 => Self::B,
                    2 => Self::C,
                    3 => Self::D,
                    4 => Self::E,
                    5 => Self::F,
                    6 => Self::G,
                    _ => unreachable!(),
                }
            }
        }

        output
    }

    fn to_number(self) -> Option<usize> {
        match self {
            ZERO => Some(0),
            ONE => Some(1),
            TWO => Some(2),
            THREE => Some(3),
            FOUR => Some(4),
            FIVE => Some(5),
            SIX => Some(6),
            SEVEN => Some(7),
            EIGHT => Some(8),
            NINE => Some(9),
            _ => None,
        }
    }
}

struct Entry {
    unique_patterns: [Segment; 10],
    output_values: [Segment; 4],
}

impl Entry {
    fn parse(line: &str) -> Self {
        let mut unique_patterns = [Segment::empty(); 10];
        let mut output_values = [Segment::empty(); 4];
        let mut entry = line.split(" | ");

        for (i, pattern) in entry.next().unwrap().split(' ').enumerate().take(10) {
            unique_patterns[i] = Segment::parse(pattern);
        }

        for (i, output) in entry.next().unwrap().split(' ').enumerate().take(4) {
            output_values[i] = Segment::parse(output);
        }

        Self {
            unique_patterns,
            output_values,
        }
    }

    fn find_value(self, permutations: &[[Segment; 7]]) -> usize {
        // Find the permutation where all of the patterns are valid
        let correct_permutation = permutations
            .iter()
            .find(|perm| {
                self.unique_patterns
                    .iter()
                    .all(|pattern| pattern.map_permutation(perm).to_number().is_some())
            })
            .unwrap();

        // Convert the output values into a number
        self.output_values.into_iter().fold(0, |acc, seg| {
            (acc * 10)
                + seg
                    .map_permutation(correct_permutation)
                    .to_number()
                    .unwrap()
        })
    }
}

fn task2(input: &[&str]) -> usize {
    // 7! = 5040, which is brute forceable
    let permutations: Vec<[Segment; 7]> = [
        Segment::A,
        Segment::B,
        Segment::C,
        Segment::D,
        Segment::E,
        Segment::F,
        Segment::G,
    ]
    .into_iter()
    .permutations(7)
    .map(|p| {
        let mut permutation_array = [Segment::empty(); 7];

        for (i, s) in p.into_iter().enumerate().take(7) {
            permutation_array[i] = s;
        }

        permutation_array
    })
    .collect();

    input
        .iter()
        .map(|line| Entry::parse(line))
        .map(|entry| entry.find_value(&permutations))
        .sum()
}

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    println!("Task 1: {}", task1(&lines));
    println!("Task 2: {}", task2(&lines));
}
