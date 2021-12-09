use std::collections::HashSet;

struct Heightmap {
    rows: Vec<Vec<u32>>,
}

impl Heightmap {
    fn parse<'a>(lines: impl Iterator<Item = &'a str>) -> Self {
        Self {
            rows: lines
                .map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect())
                .collect(),
        }
    }

    fn get_point(&self, point: (usize, usize)) -> u32 {
        let (row, column) = point;

        self.rows[row][column]
    }

    // Returns the neighbors of a point in the order up, left, down, right
    fn neighbors(&self, point: (usize, usize)) -> [Option<(usize, usize)>; 4] {
        let (row, column) = point;

        [
            row.checked_sub(1).map(|r| (r, column)),
            column.checked_sub(1).map(|c| (row, c)),
            (row + 1 < self.dimensions().0).then(|| (row + 1, column)),
            (column + 1 < self.dimensions().1).then(|| (row, column + 1)),
        ]
    }

    // Returns the height and width, in that order
    fn dimensions(&self) -> (usize, usize) {
        (self.rows.len(), self.rows[0].len())
    }
}

fn main() {
    let heightmap = Heightmap::parse(include_str!("input.txt").lines());

    let (rows, columns) = heightmap.dimensions();

    let low_points: Vec<(usize, usize)> = (0..rows)
        .flat_map(|i| (0..columns).map(move |j| (i, j)))
        .filter_map(|p| {
            heightmap
                .neighbors(p)
                .into_iter()
                .flatten()
                .all(|n| heightmap.get_point(p) < heightmap.get_point(n))
                .then(|| p)
        })
        .collect();

    let task1: u32 = low_points.iter().map(|&p| heightmap.get_point(p) + 1).sum();

    println!("Task 1: {}", task1);

    let mut basin_sizes: Vec<usize> = low_points
        .iter()
        .map(|&lp| {
            let mut basin_points: HashSet<(usize, usize)> = HashSet::new();
            let mut basin_edges: HashSet<(usize, usize)> = HashSet::new();
            basin_edges.insert(lp);

            while !basin_edges.is_empty() {
                let mut new_basin_edges = HashSet::new();

                for &p in basin_edges.iter() {
                    new_basin_edges.extend(heightmap.neighbors(p).into_iter().flatten().filter(
                        |&n| {
                            !basin_points.contains(&n)
                                && !basin_edges.contains(&n)
                                && heightmap.get_point(p) < heightmap.get_point(n)
                                && heightmap.get_point(n) != 9
                        },
                    ));

                    basin_points.insert(p);
                }

                basin_edges = new_basin_edges;
            }

            basin_points.len()
        })
        .collect();

    let third_from_end = basin_sizes.len() - 3;
    let (_, &mut third, second_and_first) = basin_sizes.select_nth_unstable(third_from_end);

    let task2 = third * second_and_first.iter().product::<usize>();
    println!("Task 2: {}", task2);
}
