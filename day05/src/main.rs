use std::cmp::max;
use std::collections::HashMap;
use std::fmt::Display;
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
    points: HashMap<Vec2, usize>,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.grid_size() {
            for x in 0..self.grid_size() {
                let point = Vec2::new(x, y);

                if let Some(&num) = self.points.get(&point) {
                    write!(f, "{}", num)?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn new() -> Self {
        Self {
            points: HashMap::new(),
        }
    }

    fn grid_size(&self) -> i32 {
        self.points.keys().map(|p| max(p.x, p.y)).max().unwrap() + 1
    }

    fn apply_line(&mut self, line: &Line) {
        for pos in line.points() {
            if let Some(p) = self.points.get_mut(&pos) {
                *p += 1;
            } else {
                self.points.insert(pos, 1);
            };
        }
    }

    fn output(&self) -> usize {
        self.points
            .values()
            .filter(|&&line_count| line_count >= 2)
            .count()
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

    if grid1.grid_size() <= 10 {
        println!("{}", grid1);
        println!("{}", grid2);
    }

    println!("Task 1: {}", grid1.output());
    println!("Task 2: {}", grid2.output());
}
