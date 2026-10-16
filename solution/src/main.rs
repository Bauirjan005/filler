mod board;
mod parser;
mod piece;
mod strategy;

#[cfg(test)]
mod tests;

use board::Player;
use parser::{format_coordinates, parse_turn};
use strategy::choose_best_move;
use std::io::{self, BufReader, Write};

fn main() {
    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin.lock());
    let mut stdout = io::stdout();

    let mut player: Option<Player> = None;

    loop {
        match parse_turn(&mut reader, &mut player) {
            Ok(Some((board, piece))) => {
                let current_player = player.unwrap_or(Player::Player1);
                let (x, y) = choose_best_move(&board, &piece, &current_player);
                let output = format_coordinates(x, y);

                if let Err(_) = stdout.write_all(output.as_bytes()) {
                    break;
                }
                if let Err(_) = stdout.flush() {
                    break;
                }
            }
            Ok(None) => {
                // End of game reached via EOF
                break;
            }
            Err(_err) => {
                // Game engine might close pipe or send unexpected exit line
                break;
            }
        }
    }
}
