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

/// The board is an 8x8 grid where each cell can contain an optional chess piece.
pub type Board = [[Option<Piece>; 8]; 8];

/// Initializes the board with chess pieces in their starting positions.
/// This function creates an 8x8 board (array) and places the white and black pieces.
pub fn init_board() -> Board {
    let mut board: Board = [[None; 8]; 8];

    // Setup white pieces on the first row (index 0)
    board[0] = [
        Some(Piece { piece_type: PieceType::Rook,   color: Color::White }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
        Some(Piece { piece_type: PieceType::Queen,  color: Color::White }),
        Some(Piece { piece_type: PieceType::King,   color: Color::White }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::White }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::White }),
        Some(Piece { piece_type: PieceType::Rook,   color: Color::White }),
    ];

    // Setup white pawns on the second row (index 1)
    board[1] = [Some(Piece { piece_type: PieceType::Pawn, color: Color::White }); 8];

    // Setup black pieces on the last row (index 7)
    board[7] = [
        Some(Piece { piece_type: PieceType::Rook,   color: Color::Black }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Queen,  color: Color::Black }),
        Some(Piece { piece_type: PieceType::King,   color: Color::Black }),
        Some(Piece { piece_type: PieceType::Bishop, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Knight, color: Color::Black }),
        Some(Piece { piece_type: PieceType::Rook,   color: Color::Black }),
    ];

    // Setup black pawns on the second-last row (index 6)
    board[6] = [Some(Piece { piece_type: PieceType::Pawn, color: Color::Black }); 8];

    board
}

/// Prints the board to the console.
/// Each cell prints the piece (if any) using a two-character representation:
/// The first character indicates the color (w or b) and the second the piece type (K, Q, P, etc.).
/// Empty squares are represented by "--".
pub fn print_board(board: &Board) {
    for row in board.iter() {
        for cell in row.iter() {
            match cell {
                Some(piece) => {
                    let piece_char = match piece.piece_type {
                        PieceType::Pawn   => "P",
                        PieceType::Knight => "N",
                        PieceType::Bishop => "B",
                        PieceType::Rook   => "R",
                        PieceType::Queen  => "Q",
                        PieceType::King   => "K",
                    };
                    let piece_color = match piece.color {
                        Color::White => "w",
                        Color::Black => "b",
                    };
                    print!("{}{} ", piece_color, piece_char);
                }
                None => print!("-- "),
            }
        }
        println!();
    }
}