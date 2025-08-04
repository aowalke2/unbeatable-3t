use std::io;

use unbeatable_3t_core::{Game, Mode, PlayerPick, State};

fn main() {
    let mode = loop {
        println!("Select mode Easy or Hard");
        let mut mode_input_line = String::new();
        io::stdin()
            .read_line(&mut mode_input_line)
            .expect("Failed to read");

        let mode = mode_input_line.trim().to_lowercase();
        match Mode::try_from(mode.as_str()) {
            Ok(mode) => break mode,
            Err(e) => println!("{}", e),
        }
    };

    let player_pick = loop {
        println!("Select X or O");
        let mut player_pick_input_line = String::new();
        io::stdin()
            .read_line(&mut player_pick_input_line)
            .expect("Failed to read");

        let player_pick = player_pick_input_line.trim().to_lowercase();
        match PlayerPick::try_from(player_pick.as_str()) {
            Ok(mode) => break mode,
            Err(e) => println!("{}", e),
        }
    };

    let mut game = Game::new(mode, player_pick);
    loop {
        println!("{}", game);
        match game.run() {
            State::GameOver(message) => {
                println!("{}", message);
                break;
            }
            State::InProgresss(message) => {
                println!("{}", message);
            }
        };

        println!()
    }
}
