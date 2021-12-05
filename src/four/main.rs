use std::collections::VecDeque;
use std::fmt::Debug;
use std::io::{self, BufRead};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() -> Result<()> {
    let mut bingo = read_input()?;

    while let Some(drawn_number) = bingo.draw_number() {
        while let Some(winner) = bingo.play(drawn_number) {
            println!(
                "board {} wins after drawing number {}",
                winner, drawn_number
            );
            print!("winning board looks like this: {:?}", bingo.boards[winner]);
            println!(
                "winning score = {}",
                bingo.score_board(winner, drawn_number)
            );
            if bingo.game_complete() {
                println!("Everyone's a winner");
                return Ok(());
            }
            println!();
        }
    }
    Err("Ran out of numbers to draw before everyone had a chance to win")?
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CellState {
    Marked,
    Unmarked,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    number: u8,
    state: CellState,
}

impl Cell {
    fn is_marked(&self) -> bool {
        self.state == CellState::Marked
    }
}

type Grid = Vec<Vec<Cell>>;

struct Board {
    grid: Grid,
    has_won: bool,
}

impl Debug for Board {
    fn fmt(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(formatter)?;
        for row in self.grid.iter() {
            write!(formatter, "[ ")?;
            for cell in row {
                match cell.state {
                    CellState::Marked => write!(formatter, "({:02}), ", cell.number)?,
                    CellState::Unmarked => write!(formatter, " {:02} , ", cell.number)?,
                }
            }
            writeln!(formatter, "]")?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Bingo {
    draw_sequence: VecDeque<u8>,
    boards: Vec<Board>,
}

impl Bingo {
    fn is_winning_grid(grid: &Grid) -> bool {
        grid.iter()
            .any(|row| row.iter().all(|cell| cell.is_marked()))
            || (0..5)
                .map(|idx| grid.iter().map(|row| row[idx]).collect::<Vec<_>>())
                .any(|col| col.iter().all(|cell| cell.is_marked()))
    }

    fn draw_number(&mut self) -> Option<u8> {
        self.draw_sequence.pop_front()
    }

    fn game_complete(&self) -> bool {
        self.boards.iter().all(|board| board.has_won)
    }

    fn play(&mut self, drawn_number: u8) -> Option<usize> {
        for (idx, board) in self.boards.iter_mut().enumerate() {
            for row in board.grid.iter_mut() {
                if let Some(cell) = row.iter_mut().find(|cell| cell.number == drawn_number) {
                    cell.state = CellState::Marked;
                }
            }
            if !board.has_won && Bingo::is_winning_grid(&board.grid) {
                board.has_won = true;
                return Some(idx);
            }
        }
        return None;
    }

    fn score_board(&self, board_idx: usize, drawn_number: u8) -> u32 {
        let board = &self.boards[board_idx];
        let sum_of_unmarked: u32 = board
            .grid
            .iter()
            .map(|row| {
                row.iter()
                    .filter_map(|cell| match cell.state {
                        CellState::Marked => None,
                        CellState::Unmarked => Some(cell.number as u32),
                    })
                    .sum::<u32>()
            })
            .sum();

        sum_of_unmarked * (drawn_number as u32)
    }
}

fn read_input() -> Result<Bingo> {
    let stdin = io::stdin();
    let mut first_line = String::new();
    stdin.read_line(&mut first_line)?;
    let draw_sequence: VecDeque<u8> = first_line
        .trim()
        .split(",")
        .map(|s| s.parse())
        .collect::<std::result::Result<_, _>>()?;
    let mut boards = vec![];
    let mut current_board = vec![];
    for (idx, line) in stdin.lock().lines().enumerate() {
        let line = line?;
        if idx % 6 != 0 {
            let nums = line
                .trim()
                .split_whitespace()
                .map(|s| {
                    s.parse().map(|number| Cell {
                        number,
                        state: CellState::Unmarked,
                    })
                })
                .collect::<std::result::Result<Vec<Cell>, _>>()?;
            current_board.push(nums);
        }
        if idx % 6 == 5 {
            assert_eq!(current_board.len(), 5);
            boards.push(Board {
                grid: current_board,
                has_won: false,
            });
            current_board = vec![];
        }
    }
    Ok(Bingo {
        draw_sequence,
        boards,
    })
}
