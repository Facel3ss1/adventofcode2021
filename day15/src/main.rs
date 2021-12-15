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

    /// Returns the neighbors to the left and above a point (in that order)
    fn neighbors(&self, point: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let (row, column) = point;

        [
            column.checked_sub(1).map(|c| (row, c)),
            row.checked_sub(1).map(|r| (r, column)),
        ]
        .into_iter()
        .flatten()
    }

    /// Returns the height and width, in that order
    fn dimensions(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }

    fn cumulative_risk(mut self) -> u32 {
        // Inspired by the seam carving algorithm
        let (rows, columns) = self.dimensions();

        for i in 0..rows {
            for j in 0..columns {
                let point = (i, j);
                let min_neighbor = self.neighbors(point).map(|n| self[n]).min();

                if let Some(min) = min_neighbor {
                    self[point] += min;
                } else {
                    // The top left point doesn't have any neighbors
                    // It doesn't contribute to the total risk, so we set it to zero
                    self[point] = 0;
                }
            }
        }

        self[(rows - 1, columns - 1)]
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
            for num in row.iter().copied() {
                write!(f, "{}", num)?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let grid = Grid::parse(include_str!("input.txt").lines());

    println!("Task 1: {}", grid.cumulative_risk());
}
