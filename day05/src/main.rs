use std::cmp::max;
use std::collections::HashSet;
use std::iter;
use std::ops::{AddAssign, Sub};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
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

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Self::Output) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
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

    fn direction(&self) -> Vec2 {
        let dir = self.b - self.a;

        // We know the lines will only be right angles or diagonals
        Vec2::new(dir.x.signum(), dir.y.signum())
    }

    fn length(&self) -> i32 {
        let dir = self.b - self.a;

        max(dir.x.abs(), dir.y.abs())
    }

    fn points(&self) -> impl Iterator<Item = Vec2> + '_ {
        let mut current_pos = self.a;

        iter::repeat_with(move || {
            let old_pos = current_pos;
            current_pos += self.direction();
            old_pos
        })
        .take(self.length() as usize + 1)
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
        for pos in line.points() {
            if self.seen_points.contains(&pos) {
                self.output_points.insert(pos);
            } else {
                self.seen_points.insert(pos);
            };
        }
    }

    fn output(&self) -> usize {
        self.output_points.len()
    }
}

fn main() {
    let lines: Vec<Line> = include_str!("input.txt").lines().map(Line::parse).collect();

    let mut grid1 = Grid::new();
    let mut grid2 = Grid::new();

    for line in lines {
        if line.is_orthogonal() {
            grid1.apply_line(&line);
        }

        grid2.apply_line(&line);
    }

    println!("Task 1: {}", grid1.output());
    println!("Task 2: {}", grid2.output());
}
