use std::{fmt, io};

use rand::{Rng, rngs::ThreadRng};
use thiserror::Error;

pub struct Game {
    board: Vec<Vec<String>>,
    rng: ThreadRng,
}

impl Game {
    pub fn new() -> Self {
        Self {
            board: vec![vec![String::from(" "); 3]; 3],
            rng: rand::rng(),
        }
    }

    pub fn is_full(&self) -> bool {
        for r in 0..3 {
            for c in 0..3 {
                if self.board[r][c] == " " {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn win(&self, value: &str) -> bool {
        if self.board[0][0] == value && self.board[1][0] == value && self.board[2][0] == value {
            return true;
        }

        if self.board[0][1] == value && self.board[1][1] == value && self.board[2][1] == value {
            return true;
        }

        if self.board[0][2] == value && self.board[1][2] == value && self.board[2][2] == value {
            return true;
        }

        if self.board[0][0] == value && self.board[0][1] == value && self.board[0][2] == value {
            return true;
        }

        if self.board[1][0] == value && self.board[1][1] == value && self.board[1][2] == value {
            return true;
        }

        if self.board[2][0] == value && self.board[2][1] == value && self.board[2][2] == value {
            return true;
        }

        if self.board[0][0] == value && self.board[1][1] == value && self.board[2][2] == value {
            return true;
        }

        if self.board[0][2] == value && self.board[1][1] == value && self.board[2][0] == value {
            return true;
        }

        return false;
    }

    fn set_position(&mut self, r: usize, c: usize, value: &str) -> Result<(), GameError> {
        if r >= 3 || c >= 3 {
            return Err(GameError::InvalidMove(r, c));
        }

        if self.board[r][c] != " " {
            return Err(GameError::InvalidMove(r, c));
        }

        self.board[r][c] = value.to_string();
        Ok(())
    }

    pub fn player_turn(&mut self, value: &str) -> Result<(), GameError> {
        println!("Pick a row...");
        let mut row_input_line = String::new();
        io::stdin()
            .read_line(&mut row_input_line)
            .expect("Failed to read");

        let r = match row_input_line.trim().parse::<usize>() {
            Ok(r) => r,
            Err(_) => return Err(GameError::InvalidInput),
        };

        println!("Pick a column...");
        let mut column_input_line = String::new();
        io::stdin()
            .read_line(&mut column_input_line)
            .expect("Failed to read");

        let c = match column_input_line.trim().parse::<usize>() {
            Ok(c) => c,
            Err(_) => return Err(GameError::InvalidInput),
        };

        match self.set_position(r, c, value) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    pub fn random_cpu_turn(&mut self, value: &str) -> Result<(), GameError> {
        let mut empty_spots = Vec::new();
        for r in 0..3 {
            for c in 0..3 {
                if self.board[r][c] == " " {
                    empty_spots.push((r, c));
                }
            }
        }

        let index = self.rng.random_range(0..empty_spots.len());
        let (r, c) = empty_spots[index];
        match self.set_position(r, c, value) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut rows = Vec::new();
        for r in 0..3 {
            let row = self.board[r].join(" | ");
            rows.push(row);
        }
        let board = rows.join("\n---------\n");
        write!(f, "{}", board)
    }
}

#[derive(Error, Debug)]
pub enum GameError {
    #[error("Invalid move")]
    InvalidMove(usize, usize),
    #[error("Must be a number!")]
    InvalidInput,
}
