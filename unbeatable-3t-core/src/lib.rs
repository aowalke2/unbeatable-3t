use std::{fmt, io};

use rand::{Rng, rngs::ThreadRng};
use thiserror::Error;

pub enum Mode {
    Easy,
    Hard,
}

impl From<&str> for Mode {
    fn from(value: &str) -> Self {
        match value {
            "easy" => Mode::Easy,
            "hard" => Mode::Hard,
            _ => panic!("Parsing to mode failed"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PlayerPick {
    X,
    O,
}

impl From<&str> for PlayerPick {
    fn from(value: &str) -> Self {
        match value {
            "X" => PlayerPick::X,
            "O" => PlayerPick::O,
            _ => panic!("Parsing to player pick failed"),
        }
    }
}

impl From<PlayerPick> for &str {
    fn from(value: PlayerPick) -> Self {
        match value {
            PlayerPick::X => "X",
            PlayerPick::O => "O",
        }
    }
}

pub enum State {
    GameOver(String),
    InProgresss(String),
}

pub struct Game {
    board: Vec<Vec<String>>,
    rng: ThreadRng,
    mode: Mode,
    is_player_turn: bool,
    player_pick: PlayerPick,
    computer_pick: PlayerPick,
}

impl Game {
    pub fn new(mode: Mode, player_pick: PlayerPick) -> Self {
        let is_player_turn = match player_pick {
            PlayerPick::X => true,
            PlayerPick::O => false,
        };

        let computer_pick = match player_pick {
            PlayerPick::X => PlayerPick::O,
            PlayerPick::O => PlayerPick::X,
        };

        Self {
            board: vec![vec![String::from(" "); 3]; 3],
            rng: rand::rng(),
            mode,
            is_player_turn,
            player_pick,
            computer_pick,
        }
    }

    pub fn run(&mut self) -> State {
        if self.is_full() {
            return State::GameOver("Game is a draw".to_string());
        }

        if self.win(self.player_pick.into()) {
            return State::GameOver("Player is the Winner!".to_string());
        }

        if self.win(self.computer_pick.into()) {
            return State::GameOver("Computer is the Winner!".to_string());
        }

        match self.is_player_turn {
            true => {
                if let Err(e) = self.player_turn() {
                    return State::InProgresss(format!("{}", e));
                }
            }
            false => {
                println!("Computer's move...");
                if let Err(e) = self.cpu_turn() {
                    return State::InProgresss(format!("{}", e));
                }
            }
        };

        self.is_player_turn = !self.is_player_turn;

        State::InProgresss("".to_string())
    }

    fn is_full(&self) -> bool {
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
        [
            // Columns
            [&self.board[0][0], &self.board[1][0], &self.board[2][0]],
            [&self.board[0][1], &self.board[1][1], &self.board[2][1]],
            [&self.board[0][2], &self.board[1][2], &self.board[2][2]],
            // Rows
            [&self.board[0][0], &self.board[0][1], &self.board[0][2]],
            [&self.board[1][0], &self.board[1][1], &self.board[1][2]],
            [&self.board[2][0], &self.board[2][1], &self.board[2][2]],
            // Diagonals
            [&self.board[0][0], &self.board[1][1], &self.board[2][2]],
            [&self.board[0][2], &self.board[1][1], &self.board[2][0]],
        ]
        .iter()
        .any(|combo| combo.iter().all(|&cell| cell == value))
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

    fn player_turn(&mut self) -> Result<(), GameError> {
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

        match self.set_position(r, c, self.player_pick.into()) {
            Ok(_) => Ok(()),
            Err(err) => Err(err),
        }
    }

    fn cpu_turn(&mut self) -> Result<(), GameError> {
        match self.mode {
            Mode::Easy => self.random_cpu_turn(self.computer_pick.into()),
            Mode::Hard => todo!(),
        }
    }

    fn random_cpu_turn(&mut self, value: &str) -> Result<(), GameError> {
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

    fn score(&self) -> u32 {
        // if self.win("X") {}
        todo!()
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
