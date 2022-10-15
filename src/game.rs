use super::*;

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    White
}

#[derive(Clone, Copy)]
pub enum CastlingAbility {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8
}

#[derive(Clone, Copy)]
pub struct Game {
    //12 bitboards
    white_pawns: Bitboard,
    white_rooks: Bitboard,
    white_knights: Bitboard,
    white_bishops: Bitboard,
    white_queens: Bitboard,
    white_king: Bitboard,
    black_pawns: Bitboard,
    black_rooks: Bitboard,
    black_knights: Bitboard,
    black_bishops: Bitboard,
    black_queens: Bitboard,
    black_king: Bitboard,

    //3 occupancy bitboards
    white_occupancies: Bitboard,
    black_ocupancies: Bitboard,
    all_occypancies: Bitboard,

    active_player: Color,
    enpassant_square: Square,
    castling_ability: u8
}

impl Game {
    pub fn new_empty () -> Self {
        Self {
            white_pawns:    Bitboard::new(),
            white_rooks:    Bitboard::new(),
            white_knights:  Bitboard::new(),
            white_bishops:  Bitboard::new(),
            white_queens:   Bitboard::new(),
            white_king:     Bitboard::new(),
            black_pawns:    Bitboard::new(),
            black_rooks:    Bitboard::new(),
            black_knights:  Bitboard::new(),
            black_bishops:  Bitboard::new(),
            black_queens:   Bitboard::new(),
            black_king:     Bitboard::new(),

            white_occupancies:  Bitboard::new(),
            black_ocupancies:   Bitboard::new(),
            all_occypancies:    Bitboard::new(),

            active_player:      Color::White,
            enpassant_square:   Square::None,
            castling_ability:   0b1111
        }
    }
}