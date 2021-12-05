use std::cmp::max;
use std::fmt::Display;
use std::ops::{AddAssign, Index, IndexMut, Sub};

#[derive(Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn parse(input: &str) -> Self {
        let mut nums = input.split(',').map(str::parse).map(Result::unwrap);

        Self {
            x: nums.next().unwrap(),
            y: nums.next().unwrap(),
        }
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
        Vec2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
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
        Vec2 {
            x: dir.x.signum(),
            y: dir.y.signum(),
        }
    }

    fn length(&self) -> i32 {
        let dir = self.b - self.a;

        max(dir.x.abs(), dir.y.abs())
    }

    fn points(&self) -> impl Iterator<Item = Vec2> {
        [self.a, self.b].into_iter()
    }
}

struct Grid {
    rows: Vec<Vec<i32>>,
}

impl Index<Vec2> for Grid {
    type Output = i32;

    fn index(&self, index: Vec2) -> &Self::Output {
        &self.rows[index.y as usize][index.x as usize]
    }
}

impl IndexMut<Vec2> for Grid {
    fn index_mut(&mut self, index: Vec2) -> &mut Self::Output {
        &mut self.rows[index.y as usize][index.x as usize]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.rows.iter() {
            for &num in row {
                if num == 0 {
                    write!(f, ".")?;
                } else {
                    write!(f, "{}", num)?;
                }
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

impl Grid {
    fn new(size: usize) -> Self {
        Self {
            rows: vec![vec![0; size]; size],
        }
    }

    fn apply_line(&mut self, line: &Line) {
        let mut current_pos = line.a;
        let dir = line.direction();

        for _ in 0..=line.length() {
            self[current_pos] += 1;
            current_pos += dir;
        }
    }

    fn output(self) -> usize {
        self.rows
            .into_iter()
            .flat_map(|row| row.into_iter())
            .filter(|&line_count| line_count >= 2)
            .count()
    }
}

fn main() {
    let lines: Vec<Line> = include_str!("input.txt").lines().map(Line::parse).collect();

    // Find the maximum extents of the grid
    let grid_size = lines
        .iter()
        .flat_map(|l| l.points())
        .map(|p| max(p.x, p.y))
        .max()
        .unwrap() as usize
        + 1;

    let mut grid1 = Grid::new(grid_size);
    let mut grid2 = Grid::new(grid_size);

    for line in lines {
        if line.is_orthogonal() {
            grid1.apply_line(&line);
        }

        grid2.apply_line(&line);
    }

    if grid_size <= 10 {
        println!("{}", grid1);
        println!("{}", grid2);
    }

    println!("Task 1: {}", grid1.output());
    println!("Task 2: {}", grid2.output());
}
