#[derive(Debug, Clone, Copy, PartialEq)]

pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

pub type Board = [[Option<Piece>; 8]; 8];

pub fn init_board() -> Board {
    let mut board: Board = [[None; 8]; 8];

    board[0] = [
        Some(Piece { piece_type: PieceType::Rook, color: Color::White }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
        Some(Piece { piece_type: PieceType::Queen, color: Color::White }),
        Some(Piece { piece_type: PieceType::King, color: Color::White }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
        Some(Piece { piece_type: PieceType::Rook, color: Color::White }),
    ];

    board[7] = [
        Some(Piece { piece_type: PieceType::Rook, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Queen, color: Color::Black }),
        Some(Piece { piece_type: PieceType::King, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Rook, color: Color::Black }),
    ];

    board[6] = [Some{Piece { piece_type: PieceType::Pawn, color: Color::White }}; 8];

    board


}

pub fn print_board(board: &Board) {
    for row in board.iter() {
    }
    for cell in row.iter() {
        match cell {
            Some(piece) => {
                let piece_char = match piece.piece_type {
                    PieceType::Pawn => 'P',
                    PieceType::Knight => 'N',
                    PieceType::Bishop => 'B',
                    PieceType::Rook => 'R',
                    PieceType::Queen => 'Q',
                    PieceType::King => 'K',
                };
                let piece_color = match piece.color {
                    Color::White => 'W',
                    Color::Black => 'B',

                };
                print!("{}{}", piece_color, piece_char);

            }
            None => print!("-- "),
}
println!();
    }
}