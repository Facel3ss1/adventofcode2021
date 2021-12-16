use std::collections::BinaryHeap;
use std::fmt::Display;
use std::ops::{Index, IndexMut};

struct DistEntry {
    point: (usize, usize),
    dist: u32,
}

impl DistEntry {
    fn new(point: (usize, usize), dist: u32) -> Self {
        Self { point, dist }
    }
}

impl PartialEq for DistEntry {
    fn eq(&self, other: &Self) -> bool {
        self.dist == other.dist
    }
}

impl Eq for DistEntry {}

impl PartialOrd for DistEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}

impl Ord for DistEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

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

    fn new_dist(size: (usize, usize)) -> Self {
        Self {
            rows: vec![vec![u32::MAX; size.1]; size.0],
        }
    }

    /// Extends the grid according to Task 2
    fn extend(&mut self) {
        let (rows, columns) = self.dimensions();

        // Add in the extra rows
        for i in rows..(rows * 5) {
            let risk_increase = (i / rows) as u32;
            let original_row = i % rows;

            let new_row = self.rows[original_row]
                .iter()
                .copied()
                .map(|risk| ((risk - 1 + risk_increase) % 9) + 1)
                .collect();

            self.rows.push(new_row);
        }

        // Add in the extra columns
        for j in columns..(columns * 5) {
            for i in 0..self.rows.len() {
                let risk_increase = (j / columns) as u32;
                let original_col = j % columns;
                let original_risk = self[(i, original_col)];

                let new_risk = ((original_risk - 1 + risk_increase) % 9) + 1;
                self.rows[i].push(new_risk);
            }
        }
    }

    /// Returns the neighbors of a point
    fn neighbors(&self, point: (usize, usize)) -> impl Iterator<Item = (usize, usize)> {
        let (row, column) = point;

        [
            row.checked_sub(1).map(|r| (r, column)),
            column.checked_sub(1).map(|c| (row, c)),
            (row + 1 < self.dimensions().0).then(|| (row + 1, column)),
            (column + 1 < self.dimensions().1).then(|| (row, column + 1)),
        ]
        .into_iter()
        .flatten()
    }

    /// Returns the height and width, in that order
    fn dimensions(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }

    fn min_risk_path(&self) -> u32 {
        // Originally, this code used something inspired by the seam carving
        // algorithm, which worked for Task 1. However it didn't work for Task
        // 2, and I realised it was because the algorithm assumed you could only
        // ever move down and to the right, which is wrong - you can move up and
        // to the left as well.

        // So I've caved and done Dijkstra's instead, which is a shame because I
        // liked my solution for Task 1 :(
        let (rows, columns) = self.dimensions();
        let target = (rows - 1, columns - 1);

        let mut dist = Grid::new_dist((rows, columns));
        dist[(0, 0)] = 0;

        let mut priority_queue: BinaryHeap<DistEntry> =
            BinaryHeap::from([DistEntry::new((0, 0), 0)]);

        while let Some(min_entry) = priority_queue.pop() {
            if min_entry.point == target {
                break;
            }

            if min_entry.dist > dist[min_entry.point] {
                continue;
            }

            for neighbor in self.neighbors(min_entry.point) {
                let alt_dist = min_entry.dist.saturating_add(self[neighbor]);

                if alt_dist < dist[neighbor] {
                    dist[neighbor] = alt_dist;
                    priority_queue.push(DistEntry::new(neighbor, dist[neighbor]));
                }
            }
        }

        dist[target]
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
    let mut grid = Grid::parse(include_str!("input.txt").lines());

    println!("Task 1: {}", grid.min_risk_path());

    grid.extend();
    println!("Task 2: {}", grid.min_risk_path());
}
