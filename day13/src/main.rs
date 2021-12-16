use std::collections::HashSet;
use std::fmt::Display;

enum Fold {
    Horizontal(usize),
    Vertical(usize),
}

impl Fold {
    fn parse(input: &str) -> Self {
        let mut input = input.strip_prefix("fold along ").unwrap().split('=');
        let axis = input.next().unwrap();
        let position = str::parse(input.next().unwrap()).unwrap();

        match axis {
            "y" => Self::Horizontal(position),
            "x" => Self::Vertical(position),
            _ => panic!(),
        }
    }
}

impl Display for Fold {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Fold::Horizontal(pos) => write!(f, "y={}", pos),
            Fold::Vertical(pos) => write!(f, "x={}", pos),
        }
    }
}

struct Paper {
    /// The points on the paper
    points: HashSet<(usize, usize)>,
    /// The extents of the paper, exclusive
    extents: (usize, usize),
}

impl Paper {
    fn parse<'a>(input: impl Iterator<Item = &'a str>) -> Self {
        let mut points = HashSet::new();

        for line in input {
            let mut numbers = line.split(',').map(str::parse).map(Result::unwrap);

            points.insert((numbers.next().unwrap(), numbers.next().unwrap()));
        }

        let max_x = points.iter().copied().map(|(x, _)| x).max().unwrap() + 1;
        let max_y = points.iter().copied().map(|(_, y)| y).max().unwrap() + 1;
        let extents = (max_x, max_y);

        Self { points, extents }
    }

    fn apply_fold(&mut self, fold: Fold) {
        // println!("Folding {:?} paper along {}", self.extents, fold);

        // Change the extents to the folded paper
        match fold {
            Fold::Horizontal(pos) => self.extents.1 = pos,
            Fold::Vertical(pos) => self.extents.0 = pos,
        }

        // Take out all of the points outside the extents
        let outside_extents: HashSet<(usize, usize)> = self
            .points
            .iter()
            .copied()
            .filter(|p| !self.within_extents(p))
            .collect();

        self.points = self.points.difference(&outside_extents).copied().collect();

        // Fold the points outside the extents
        let folded_points = outside_extents.into_iter().map(|(x, y)| match fold {
            Fold::Horizontal(pos) => (x, (2 * pos) - y),
            Fold::Vertical(pos) => ((2 * pos) - x, y),
        });

        self.points.extend(folded_points);
    }

    fn within_extents(&self, point: &(usize, usize)) -> bool {
        let (width, height) = self.extents;
        let &(x, y) = point;

        x < width && y < height
    }

    fn num_points(&self) -> usize {
        self.points.len()
    }
}

impl Display for Paper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (max_x, max_y) = self.extents;

        for y in 0..max_y {
            for x in 0..max_x {
                if self.points.contains(&(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let (fold_input, paper_input): (Vec<&str>, Vec<&str>) = include_str!("input.txt")
        .lines()
        .filter(|&l| !l.is_empty())
        .partition(|&l| l.starts_with("fold along "));

    let mut paper = Paper::parse(paper_input.into_iter());
    let mut folds = fold_input.into_iter().map(Fold::parse);

    paper.apply_fold(folds.next().unwrap());

    println!("Task 1: {}", paper.num_points());
    println!("Task 2:");

    folds.for_each(|f| paper.apply_fold(f));

    println!("{}", paper);
}
