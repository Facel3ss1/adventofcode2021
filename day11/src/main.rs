use std::collections::HashSet;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

struct Grid {
    rows: Vec<Vec<u32>>,
}

impl Grid {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        Self {
            rows: lines
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    // Returns the neighbors of a point, including diagonals
    fn neighbors(&self, point: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let (rows, columns) = self.dimensions();
        let (row, column) = point;

        [
            row.checked_sub(1),
            Some(row),
            (row + 1 < rows).then(|| row + 1),
        ]
        .into_iter()
        .flat_map(move |i| {
            [
                column.checked_sub(1),
                Some(column),
                (column + 1 < columns).then(|| column + 1),
            ]
            .into_iter()
            .flat_map(move |j| i.zip(j))
        })
        .filter(move |&p| p != point)
    }

    // Returns the height and width, in that order
    fn dimensions(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }
}

impl Index<(usize, usize)> for Grid {
    type Output = u32;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let (row, column) = index;

        &self.rows[row][column]
    }
}

impl IndexMut<(usize, usize)> for Grid {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let (row, column) = index;

        &mut self.rows[row][column]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for &num in row.iter() {
                write!(f, "{}", num)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut grid = Grid::parse(include_str!("input.txt").lines());
    let (rows, columns) = grid.dimensions();
    let num_points = rows * columns;

    let mut task1 = 0;
    let mut step = 1;

    let task2 = loop {
        // Start by increasing all of the points
        let mut to_increase: Vec<(usize, usize)> = (0..rows)
            .flat_map(|i| (0..columns).map(move |j| (i, j)))
            .collect();
        // Keep track of the points that have flashed already
        let mut flashed_points = HashSet::new();

        // Keep going until we don't have any more points to increase
        while !to_increase.is_empty() {
            let mut new_to_increase = Vec::new();

            for point in to_increase.drain(..) {
                grid[point] += 1;

                // If this is a flashed point, increase its neighbors next time
                if grid[point] > 9 && flashed_points.insert(point) {
                    new_to_increase.extend(grid.neighbors(point));
                }
            }

            to_increase = new_to_increase;
        }

        // Task 1 only cares about the first 100 steps
        if (1..=100).contains(&step) {
            task1 += flashed_points.len();
        }

        // If all of the points have flashed, we've finished Task 2
        if flashed_points.len() == num_points {
            break step;
        }

        // Reset all of the flashed points back to zero
        for flashed_point in flashed_points {
            grid[flashed_point] = 0;
        }

        step += 1;

        // println!("{}", grid);
    };

    println!("Task 1: {}", task1);
    println!("Task 2: {}", task2);
}
