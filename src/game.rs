use super::*;

#[derive(PartialEq)]
pub enum Color {
    Black,
    White
}

#[derive(Clone, Copy)]
pub struct Game {
    white_pawns: BitBoard,
    white_rooks: BitBoard,
    white_knights: BitBoard,
    white_bishops: BitBoard,
    white_queens: BitBoard,
    black_pawns: BitBoard,
    black_rooks: BitBoard,
    black_knights: BitBoard,
    black_bishops: BitBoard,
    black_queens: BitBoard,

    white_king_position: u8,
    black_king_position: u8,
}

impl Game {
    pub fn new_empty () -> Self {
        Self {
            white_pawns: BitBoard::from_u64(0),
            white_rooks: BitBoard::from_u64(0),
            white_knights: BitBoard::from_u64(0),
            white_bishops: BitBoard::from_u64(0),
            white_queens: BitBoard::from_u64(0),
            black_pawns: BitBoard::from_u64(0),
            black_rooks: BitBoard::from_u64(0),
            black_knights: BitBoard::from_u64(0),
            black_bishops: BitBoard::from_u64(0),
            black_queens: BitBoard::from_u64(0),

            white_king_position: 60,
            black_king_position: 4,
        }
    }
}