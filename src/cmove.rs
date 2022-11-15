use std::fmt::{Display, self, write};

use super::*;

pub const NULL_MOVE: Move = Move { data: 0, score: 0 };

#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub struct Move {
    data: u32,
    pub score: u8
}

impl Move {
    #[cfg(test)]
    pub fn new_friendly(from_square: Square,        // 0x3f
                        to_square: Square,          // 0xfc0
                        piece: Piece,               // 0xf000
                        promotion: Piece,           // 0xf0000
                        is_capture: bool,           // 0x100000
                        is_double_push: bool,       // 0x200000
                        is_enpassant: bool,         // 0x400000
                        is_castling: bool           // 0x800000) 
                    ) -> Self {
        Move::new(from_square as u8, to_square as u8, piece as u8, promotion as u8, is_capture, is_double_push, is_enpassant, is_castling)
    }

    pub fn new(from_square: u8,        // 0x3f
               to_square: u8,          // 0xfc0
               piece: u8,               // 0xf000
               promotion: u8,           // 0xf0000
               is_capture: bool,           // 0x100000
               is_double_push: bool,       // 0x200000
               is_enpassant: bool,         // 0x400000
               is_castling: bool           // 0x800000
            ) -> Self {
        
        let mut data: u32 = from_square as u32;

        data |= (to_square as u32) << 6 | (piece as u32) << 12 | (promotion as u32) << 16;
        if is_capture { data |= 0x100000; }
        if is_double_push { data |= 0x200000; }
        if is_enpassant { data |= 0x400000; }
        if is_castling { data |= 0x800000; }

        Self { data: data, score: 0 }
    }

    pub fn set_score(&mut self, score: u8) {
        self.score = score;
    }

    pub fn from_square(&self) -> u8 {
        (self.data & 0x3f) as u8
    }

    pub fn to_square(&self) -> u8 {
        ((self.data & 0xfc0) >> 6) as u8
    }

    pub fn piece(&self) -> u8 {
        ((self.data & 0xf000) >> 12) as u8
    }

    pub fn promotion(&self) -> u8 {
        ((self.data & 0xf0000) >> 16) as u8
    }

    pub fn is_capture(&self) -> bool {
        (self.data & 0x100000) == 0x100000
    }

    pub fn is_double_push(&self) -> bool {
        (self.data & 0x200000) == 0x200000
    }

    pub fn is_enpassant(&self) -> bool {
        (self.data & 0x400000) == 0x400000
    }

    pub fn is_castling(&self) -> bool {
        (self.data & 0x800000) == 0x800000
    }

    #[cfg(test)]
    pub fn print(&self) {
        print!(" From: {}", SQUARE_STRINGS[self.from_square() as usize]);
        print!("    To: {}", SQUARE_STRINGS[self.to_square() as usize]);
        print!("    Piece: {}", PIECE_STRINGS[self.piece() as usize]);
        if self.is_enpassant() {
            print!("    Enpassant capture")
        }
        else if self.is_capture() {
            print!("    Capture")
        }
        if self.promotion() != Piece::None as u8 {
            print!("    Promoted to: {}", PIECE_STRINGS[self.promotion() as usize])
        }
        if self.is_castling() {
            print!("    Castling")
        }
        if self.is_double_push() {
            print!("    Double push")
        }
        println!()
    }
}

impl Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", SQUARE_STRINGS[self.from_square() as usize].to_string());
        write!(f, "{}", SQUARE_STRINGS[self.to_square() as usize]);
        if self.promotion() != Piece::None as u8 {
            write!(f, "{}", PIECE_STRINGS[self.promotion() as usize].to_lowercase())
        } else {
            write!(f, "")
        }
    }
}