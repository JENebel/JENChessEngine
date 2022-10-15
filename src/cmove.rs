use super::*;

#[derive(Clone, Copy)]
pub struct Move {
    data: u32
}

impl Move {
    pub fn new(from_square: Square,        // 0x3f
               to_square: Square,          // 0xfc0
               captured_piece: Piece,     // 0xf000
               promoted_piece: Piece,     // 0xf0000
               is_capture: bool,           // 0x100000
               is_double_push: bool,       // 0x200000
               is_enpassant: bool,         // 0x400000
               is_castling: bool           // 0x800000
            ) -> Self {
        
        let mut data: u32 = from_square as u32;
        
        data |= (to_square as u32) << 6;
        data |= (promoted_piece as u32) << 16;
        if is_capture { 
            data |= 0x100000; 
            data |= (captured_piece as u32) << 12; 
        }
        if is_double_push { data |= 0x200000; }
        if is_enpassant { data |= 0x400000; }
        if is_castling { data |= 0x800000; }

        Self { data: data }
    }

    pub fn from_square(&self) -> u8 {
        (self.data & 0x3f) as u8
    }

    pub fn to_square(&self) -> u8 {
        ((self.data & 0xfc0) >> 6) as u8
    }

    pub fn captured_piece(&self) -> u8 {
        ((self.data & 0xf000) >> 12) as u8
    }

    pub fn promoted_piece(&self) -> u8 {
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

    pub fn print(&self) {
        print!(" From: {}", SQUARE_STRINGS[self.from_square() as usize]);
        print!("    To: {}", SQUARE_STRINGS[self.to_square() as usize]);
        if self.is_capture() {
            if self.is_enpassant() {
                print!("    Enpassant capture")
            }
            print!("    Captured: {}", PIECE_STRINGS[self.captured_piece() as usize])
        }
        if self.to_square() / 8 == 0 || self.to_square() / 8 == 7 {
            print!("    Promoted: {}", PIECE_STRINGS[self.promoted_piece() as usize])
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