use bitflags::bitflags;

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
    /// A seven bit representation of a segment pattern
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

impl Segment {
    /// Parses the string representation into the bit representation
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

    /// Returns the length of the string representation of this pattern
    fn len(&self) -> u32 {
        self.bits.count_ones()
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

    fn find_value(self) -> usize {
        let &one = self.unique_patterns.iter().find(|s| s.len() == 2).unwrap();
        let &four = self.unique_patterns.iter().find(|s| s.len() == 4).unwrap();

        self.output_values
            .into_iter()
            .map(
                // Credit to u/frankbsad for this insane solution
                |output| match (output.len(), (output & one).len(), (output & four).len()) {
                    (6, 2, 3) => 0,
                    (2, 2, 2) => 1,
                    (5, 1, 2) => 2,
                    (5, 2, 3) => 3,
                    (4, 2, 4) => 4,
                    (5, 1, 3) => 5,
                    (6, 1, 3) => 6,
                    (3, 2, 2) => 7,
                    (7, 2, 4) => 8,
                    (6, 2, 4) => 9,
                    (_, _, _) => panic!(),
                },
            )
            .fold(0, |acc, num| acc * 10 + num)
    }
}

fn task2(input: &[&str]) -> usize {
    input
        .iter()
        .map(|line| Entry::parse(line).find_value())
        .sum()
}

fn main() {
    let lines: Vec<&str> = include_str!("input.txt").lines().collect();
    println!("Task 1: {}", task1(&lines));
    println!("Task 2: {}", task2(&lines));
}
