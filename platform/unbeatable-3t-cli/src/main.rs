use std::io;

use unbeatable_3t_core::{Game, State};

fn main() {
    println!("Select mode Easy or Hard");
    let mut mode_input_line = String::new();
    io::stdin()
        .read_line(&mut mode_input_line)
        .expect("Failed to read");

    println!("Select X or O");
    let mut player_pick_input_line = String::new();
    io::stdin()
        .read_line(&mut player_pick_input_line)
        .expect("Failed to read");

    let mut game = Game::new(
        mode_input_line.trim().to_lowercase().as_str().into(),
        player_pick_input_line.trim().to_uppercase().as_str().into(),
    );

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
