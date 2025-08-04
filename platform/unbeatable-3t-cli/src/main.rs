use unbeatable_3t_core::Game;

fn main() {
    let mut game = Game::new();
    let mut is_player_turn = true;

    loop {
        println!("{}", game);
        if game.is_full() {
            println!("Game is a Draw!");
            break;
        }

        if game.win("X") {
            println!("X is the Winner!");
            break;
        }

        if game.win("O") {
            println!("O is the Winner!");
            break;
        }

        let value = match is_player_turn {
            true => "X",
            false => "O",
        };

        match is_player_turn {
            true => {
                if let Err(e) = game.player_turn(value) {
                    println!("{}", e);
                    continue;
                }
            }
            false => {
                println!("Computer's move...");
                if let Err(e) = game.random_cpu_turn(value) {
                    println!("{}", e);
                    continue;
                }
            }
        };

        is_player_turn = !is_player_turn;
        println!()
    }
}
