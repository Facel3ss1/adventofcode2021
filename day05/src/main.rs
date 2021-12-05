use derive_more::{AddAssign, Sub};

use std::cmp::max;
use std::collections::HashSet;
use std::iter;

#[derive(Clone, Copy, PartialEq, Eq, Hash, AddAssign, Sub)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn parse(input: &str) -> Self {
        let mut nums = input.split(',').map(str::parse).map(Result::unwrap);

        Self::new(nums.next().unwrap(), nums.next().unwrap())
    }
}

struct Line {
    a: Vec2,
    b: Vec2,
}

impl Line {
    fn parse(input: &str) -> Self {
        let mut points = input.split(" -> ").map(Vec2::parse);

        Self {
            a: points.next().unwrap(),
            b: points.next().unwrap(),
        }
    }

    // Is this line at a right angle (horizontal/vertical)
    fn is_orthogonal(&self) -> bool {
        self.a.x == self.b.x || self.a.y == self.b.y
    }

    // A unit vector of the line's direction
    fn direction(&self) -> Vec2 {
        let dir = self.b - self.a;

        // We know the lines will only be right angles or diagonals
        Vec2::new(dir.x.signum(), dir.y.signum())
    }

    // The number of points on the line
    fn length(&self) -> usize {
        let dir = self.b - self.a;

        max(dir.x.abs(), dir.y.abs()) as usize + 1
    }

    // An iterator of all the points on the line
    fn points(&self) -> impl Iterator<Item = Vec2> + '_ {
        let mut current_pos = self.a;

        iter::repeat_with(move || {
            let old_pos = current_pos;
            current_pos += self.direction();

            old_pos
        })
        .take(self.length())
    }
}

struct Grid {
    seen_points: HashSet<Vec2>,
    output_points: HashSet<Vec2>,
}

impl Grid {
    fn new() -> Self {
        Self {
            seen_points: HashSet::new(),
            output_points: HashSet::new(),
        }
    }

    fn apply_line(&mut self, line: &Line) {
        for point in line.points() {
            if !self.seen_points.insert(point) {
                // The point was already in seen_points
                self.output_points.insert(point);
            }
        }
    }

    fn output(&self) -> usize {
        self.output_points.len()
    }
}

fn main() {
    let (orthogonal_lines, diagonal_lines): (Vec<Line>, Vec<Line>) = include_str!("input.txt")
        .lines()
        .map(Line::parse)
        .partition(|l| l.is_orthogonal());

    let mut grid = Grid::new();

    for line in orthogonal_lines {
        grid.apply_line(&line);
    }

    println!("Task 1: {}", grid.output());

    for line in diagonal_lines {
        grid.apply_line(&line);
    }

    println!("Task 2: {}", grid.output());
}
