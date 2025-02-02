mod chess;
mod game;

use crate::game::Game;
use crate::chess::print_board;


fn main() {
    let mut game = Game::new();

    println!("Welcome to Chess Made By M0nke!");
    print_board(&game.board);

    println!("\nAttempting to move a white pawn from (1,0) to (2,0)...");
    match game.make_move((1,0), (2,0)) {
        Ok(_) => println!("Move successful!"),
        Err(e) => println!("Move failed: {}", e),
    }


    println!("\nCurrent board state:");
    print_board(&game.board);
}
