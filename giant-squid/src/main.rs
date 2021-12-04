use std::{
    fmt::Display,
    io::{stdin, Read},
    str::FromStr,
};

#[derive(Debug, Clone)]
enum Cell {
    Marked(usize),
    Unmarked(usize),
}

#[derive(Debug, Clone)]
struct Board(Vec<Vec<Cell>>);

impl Board {
    fn mark_number(&mut self, n: usize) {
        for row in self.0.iter_mut() {
            for cell in row.iter_mut() {
                if let Cell::Unmarked(c) = cell {
                    if n == *c {
                        *cell = Cell::Marked(n);
                    }
                }
            }
        }
    }

    fn check_win(&self) -> bool {
        self.0.iter().any(|row| {
            row.iter().all(|cell| {
                if let Cell::Marked(_) = cell {
                    true
                } else {
                    false
                }
            })
        }) || (0..self.0[0].len()).any(|col| {
            (0..self.0.len()).all(|row| {
                if let Cell::Marked(_) = self.0[row][col] {
                    true
                } else {
                    false
                }
            })
        })
    }

    fn sum_unmarked(&self) -> usize {
        let mut sum = 0;
        for row in self.0.iter() {
            for cell in row {
                if let Cell::Unmarked(n) = cell {
                    sum += n;
                }
            }
        }
        return sum;
    }
}

impl FromStr for Board {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = vec![];
        for row in s.split("\n") {
            let mut board_row = vec![];
            for cell in row.split(" ").filter(|s| !s.is_empty()) {
                board_row.push(Cell::Unmarked(cell.parse::<usize>().unwrap()));
            }
            board.push(board_row);
        }
        Ok(Board(board))
    }
}

fn main() {
    let mut input = String::new();
    stdin().lock().read_to_string(&mut input).unwrap();

    let mut parts = input.split("\n\n");
    let drawn_numbers = parts.next().unwrap();
    let mut boards = parts
        .map(|s| s.parse::<Board>().unwrap())
        .collect::<Vec<_>>();

    let mut last_drawn_board = None;

    for drawn in drawn_numbers
        .split(",")
        .map(|s| s.parse::<usize>().unwrap())
    {
        let mut unfinished_boards = boards
            .iter_mut()
            .filter(|b| !b.check_win())
            .collect::<Vec<_>>();
        for board in unfinished_boards.iter_mut() {
            board.mark_number(drawn);
            if board.check_win() {
                if last_drawn_board.is_none() {
                    println!("Part 1: {}", drawn * board.sum_unmarked());
                }
                last_drawn_board = Some((drawn, board.clone()))
            }
        }
    }
    let (drawn, board) = last_drawn_board.unwrap();
    println!("Part 2: {}", drawn * board.sum_unmarked());
}

// This is extra for better debugging
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}",
            self.0
                .iter()
                .map(|row| row
                    .iter()
                    .map(|cell| match cell {
                        Cell::Marked(n) => format!("{: >2}", n),
                        // Cell::Unmarked(n) => format!(" {: >2} ", n),
                        Cell::Unmarked(_) => format!("__"),
                    })
                    .collect::<Vec<_>>()
                    .join(" "))
                .collect::<Vec<_>>()
                .join("\n")
        ))
    }
}
