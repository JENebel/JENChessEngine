pub struct Move {
    data: u32
}

impl Move {
    pub fn new(from_square: u32,        // 0x3f
               to_square: u32,          // 0xfc0
               captured_piece: u32,     // 0xf000
               promoted_piece: u32,     // 0xf0000
               is_capture: bool,           // 0x100000
               is_double_push: bool,       // 0x200000
               is_enpassant: bool,         // 0x400000
               is_castling: bool           // 0x800000
            ) -> Self {
        
        let mut data: u32 = from_square as u32;
        
        data |= to_square << 6;
        data |= promoted_piece << 16;
        if is_capture { 
            data |= 0x100000; 
            data |= captured_piece << 12; 
        }
        if is_double_push { data |= 0x200000; }
        if is_enpassant { data |= 0x400000; }
        if is_castling { data |= 0x800000; }

        Self { data: data }
    }

    pub fn from_square(&mut self) -> u8 {
        (self.data & 0x3f) as u8
    }

    pub fn to_square(&mut self) -> u8 {
        ((self.data & 0xfc0) >> 6) as u8
    }

    pub fn captured_piece(&mut self) -> u8 {
        ((self.data & 0xf000) >> 12) as u8
    }

    pub fn promoted_piece(&mut self) -> u8 {
        ((self.data & 0xf0000) >> 16) as u8
    }

    pub fn is_capture(&mut self) -> bool {
        (self.data & 0x100000) == 0x100000
    }

    pub fn is_double_push(&mut self) -> bool {
        (self.data & 0x200000) == 0x200000
    }

    pub fn is_enpassant(&mut self) -> bool {
        (self.data & 0x400000) == 0x400000
    }

    pub fn is_castling(&mut self) -> bool {
        (self.data & 0x800000) == 0x800000
    }
}