use std::fs::File;
use std::io::{BufRead, BufReader, Read};
#[derive(Copy, Clone)]
pub struct Cell {
    pub num: u32,
    pub marked: bool,
}
impl Cell {
    fn new(num: u32) -> Cell {
        Cell { num, marked: false }
    }
}
struct Bingo {
    board: Vec<Vec<Cell>>,
    won: bool,
    score: u32,
}

impl Bingo {
    fn mark(&mut self, num: u32) {
        for row in &mut self.board {
            if let Some(cell) = row.iter_mut().find(|x| x.num == num) {
                cell.marked = true;
                break;
            }
        }
        if self.won {
            return;
        }
        self.check_won();

        if self.won {
            self.score = num
                * self
                    .board
                    .iter()
                    .map(|row| {
                        row.iter()
                            .filter(|cell| !cell.marked)
                            .fold(0, |acc, elem| acc + elem.num)
                    })
                    .sum::<u32>();
            println!("A board has won with score {}", self.score);
        }
    }

    fn check_won(&mut self) {
        if self
            .board
            .iter()
            .any(|row| row.iter().all(|cell| cell.marked))
        {
            self.won = true;
            return;
        }
        for i in 0..self.board[0].len() {
            let column = column(&self.board, i);
            if column.iter().all(|cell| cell.marked) {
                self.won = true;
                break;
            }
        }
    }
}
fn column(board: &Vec<Vec<Cell>>, n: usize) -> Vec<Cell> {
    board.iter().map(|v| v[n]).collect()
}
fn read_numbers(filename: &str, buffer: &mut String) -> Vec<u32> {
    let file = File::open(filename).expect("File wasn't found");
    let mut reader = BufReader::new(file);

    let mut numbers = String::new();
    reader.read_line(&mut numbers).unwrap();

    reader.read_to_string(buffer);

    numbers
        .strip_suffix('\n')
        .unwrap_or(numbers.as_str())
        .split(',')
        .map(|c| c.parse::<u32>().unwrap())
        .collect()
}
fn string_to_board(board: &str) -> Bingo {
    let board: Vec<Vec<Cell>> = board
        .split('\n')
        .map(|c| {
            c.split_whitespace()
                .map(|s| Cell::new(s.parse::<u32>().unwrap()))
                .collect()
        })
        .collect();
    Bingo {
        board,
        won: false,
        score: 0,
    }
}
fn main() {
    let mut boards = String::new();
    let numbers = read_numbers("input.txt", &mut boards);

    let boards: Vec<&str> = boards
        .strip_prefix('\n')
        .unwrap_or(boards.as_str())
        .split("\n\n")
        .collect();

    let mut bingo_boards: Vec<Bingo> = boards
        // split each board into lines and split each line into cells
        .iter()
        .map(|board| string_to_board(board))
        .collect();
    for num in numbers {
        bingo_boards.iter_mut().for_each(|board| board.mark(num));
    }
}
