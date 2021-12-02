#[derive(Debug)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn new() -> Self {
        Self {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn apply_movement_task1(&mut self, mov: Movement) {
        match mov {
            Movement::Forward(dist) => self.horizontal += dist,
            Movement::Up(dist) => self.depth -= dist,
            Movement::Down(dist) => self.depth += dist,
        }
    }

    fn apply_movement_task2(&mut self, mov: Movement) {
        match mov {
            Movement::Forward(dist) => {
                self.horizontal += dist;
                self.depth += self.aim * dist;
            }
            Movement::Up(dist) => self.aim -= dist,
            Movement::Down(dist) => self.aim += dist,
        }
    }
}

#[derive(Clone, Copy)]
enum Movement {
    Forward(i32),
    Up(i32),
    Down(i32),
}

fn main() {
    let mut pos1 = Position::new();
    let mut pos2 = Position::new();

    let movements = include_str!("input.txt").lines().map(|l| {
        let mut tokens = l.split(' ');

        let dir = tokens.next().expect("No direction found");
        let dist: i32 = tokens
            .next()
            .expect("No distance found")
            .parse()
            .expect("Failed to parse distance");

        match dir {
            "forward" => Movement::Forward(dist),
            "up" => Movement::Up(dist),
            "down" => Movement::Down(dist),
            _ => panic!("Unexpected direction for line {}", l),
        }
    });

    for mov in movements {
        pos1.apply_movement_task1(mov);
        pos2.apply_movement_task2(mov);
    }

    println!("Task 1 Final position: {:?}", pos1);
    println!("Task 1: {}", pos1.horizontal * pos1.depth);
    println!("Task 2 Final position: {:?}", pos2);
    println!("Task 2: {}", pos2.horizontal * pos2.depth);
}
