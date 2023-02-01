use std::fmt::Display;

use crate::position::{SQUARE_STRINGS, Piece};

#[derive(Copy, Clone)]
pub struct Move {
    pub from_sq: u8,
    pub to_sq: u8,
    pub piece: Piece,
    pub promotion: Piece,
    pub is_capture: bool,
    pub is_double_push: bool,
    pub is_enpassant: bool,
    pub is_castling: bool,

    pub score: u16,
}

impl Move {
    pub fn new(
        from_sq: u8,
        to_sq: u8,
        piece: Piece,
        promotion: Piece,
        is_capture: bool,
        is_double_push: bool,
        is_enpassant: bool,
        is_castling: bool) -> Self {
            Self {
                from_sq,
                to_sq,
                piece,
                promotion,
                is_capture,
                is_double_push,
                is_enpassant,
                is_castling,
                score: u16::MAX,
            }
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} @ {} -> {}", self.piece, SQUARE_STRINGS[self.from_sq as usize], SQUARE_STRINGS[self.to_sq as usize])
    }
}

impl Default for Move {
    fn default() -> Self {
        Self { 
            from_sq:        Default::default(),
            to_sq:          Default::default(),
            piece:          Default::default(),
            promotion:      Default::default(),
            is_capture:     Default::default(),
            is_double_push: Default::default(),
            is_enpassant:   Default::default(),
            is_castling:    Default::default(),
            score:          Default::default(),
        }
    }
}