use crate::chess::{Board, init_board, Color};

pub struct Game {
    pub board: Board, 
    pub turn: Color,

}

impl Game {
    pub fn new() -> Self {
        Self {
            board: init_board(),
            turn: Color::White,
        }
    }

    pub fn make_move(&mut self, from: (usize, usize), to: (usize, usize)) -> Result<(), String> {
        if from.0 >= 8 || from.1 >= 8 || to.0 >= 8 || to.1 >= 8{
            return Err("Invalid Board Position".into());
        }

        let piece_option = self.board[from.0][from.1];
        if piece_option.is_none() {
            return Err("No piece at the given position".into());
        }
        let piece = piece_option.unwrap();

        if piece.color != self.turn {
            return Err("It is not your turn".into());
        }

        self.board[to.0][to.1] = Some(piece);
        self.board[from.0][from.1] = None;

        self.turn = if self.turn == Color::White { Color::Black } else { Color::White };

        Ok(())
    }

}