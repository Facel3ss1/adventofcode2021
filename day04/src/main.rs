struct Board {
    rows: [[(i32, bool); 5]; 5],
}

impl Board {
    fn parse(lines: &[&str]) -> Self {
        let mut board = Self {
            rows: [[(-1, false); 5]; 5],
        };

        for (i, line) in lines.iter().enumerate().take(5) {
            let row_numbers = line.split_whitespace().map(str::parse).map(Result::unwrap);

            for (j, num) in row_numbers.into_iter().enumerate().take(5) {
                board.rows[i][j].0 = num;
            }
        }

        board
    }

    fn mark_number(&mut self, num: i32) {
        for i in 0..5 {
            for j in 0..5 {
                if self.rows[i][j].0 == num {
                    self.rows[i][j].1 = true;
                }
            }
        }
    }

    fn marked_rows(&self) -> [[bool; 5]; 5] {
        self.rows.map(|row| row.map(|(_, marked)| marked))
    }

    fn marked_columns(&self) -> [[bool; 5]; 5] {
        [0, 1, 2, 3, 4].map(|i| self.rows.map(|row| row[i].1))
    }

    fn has_won(&self) -> bool {
        let completed_row = self
            .marked_rows()
            .iter()
            .any(|row| row.iter().copied().all(|m| m));

        let completed_col = self
            .marked_columns()
            .iter()
            .any(|col| col.iter().copied().all(|m| m));

        completed_row || completed_col
    }

    fn score(&self, just_called: i32) -> i32 {
        let sum_unmarked: i32 = self
            .rows
            .iter()
            .flat_map(|row| row.iter())
            .filter(|(_, marked)| !marked)
            .map(|(num, _)| num)
            .sum();

        sum_unmarked * just_called
    }
}

fn main() {
    let mut lines = include_str!("input.txt").lines();

    let numbers: Vec<i32> = lines
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .map(Result::unwrap)
        .collect();

    let lines: Vec<&str> = lines.filter(|&l| !l.is_empty()).collect();
    // On nightly there's an array_chunks() method that would have worked nicely
    let mut boards: Vec<Board> = lines.chunks(5).map(Board::parse).collect();
    let mut finished_boards: Vec<(Board, i32)> = Vec::with_capacity(boards.len());

    for num in numbers {
        boards.iter_mut().for_each(|b| b.mark_number(num));

        while let Some(board_idx) = boards.iter().position(|b| b.has_won()) {
            finished_boards.push((boards.remove(board_idx), num));
        }
    }

    let (task1_board, task1_num) = &finished_boards[0];
    let (task2_board, task2_num) = &finished_boards[finished_boards.len() - 1];

    println!("Task 1 Answer: {}", task1_board.score(*task1_num));
    println!("Task 2 Answer: {}", task2_board.score(*task2_num));
}
