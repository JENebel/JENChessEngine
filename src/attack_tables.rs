use crate::{game::Color, bit_board::BitBoard};

const WHITE_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(true);
const BLACK_PAWN_ATTACKS: [u64; 64] = generate_pawn_attacks(false);
const KNIGHT_ATTACKS: [u64; 64] = generate_knight_attacks();
const KING_ATTACKS: [u64; 64] = generate_king_attacks();

const fn generate_pawn_attacks(color: bool) -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if color {
                if file != 7 { result = result | base >> 7 as u64 }
                if file != 0 { result = result | base >> 9 as u64 }
                
            } else {
                if file != 7 { result = result | base << 7 as u64 }
                if file != 0 { result = result | base << 9 as u64 }
            }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_knight_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 1 && file < 7 { result = result | base >> 15 as u64 }
            if rank > 0 && file < 6 { result = result | base >> 6 as u64 }

            if rank < 7 && file < 6 { result = result | base << 10 as u64 }
            if rank < 6 && file < 7 { result = result | base << 17 as u64 }

            if rank > 1 && file > 0 { result = result | base >> 17 as u64 }
            if rank > 0 && file > 1 { result = result | base >> 10 as u64 }

            if rank < 7 && file > 1 { result = result | base << 6 as u64 }
            if rank < 6 && file > 0 { result = result | base << 15 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

const fn generate_king_attacks() -> [u64; 64] {
    let mut attacks = [0; 64];

    let mut index = 0;

    let mut rank: u8 = 0;
    while rank < 8 {
        let mut file: u8 = 0;
        while file < 8 {
            let base: u64 = 1 << (rank*8+file);
            let mut result: u64 = 0;
            
            if rank > 0 { result = result | base >> 8 as u64 }
            if file > 0 { result = result | base >> 1 as u64 }
            if rank < 7 { result = result | base << 8 as u64 }
            if file < 7 { result = result | base << 1 as u64 }

            if file > 0 && rank > 0 { result = result | base >> 9 as u64 }
            if file < 7 && rank > 0 { result = result | base >> 7 as u64 }
            if file > 0 && rank < 7 { result = result | base << 7 as u64 }
            if file < 7 && rank < 7 { result = result | base << 9 as u64 }

            attacks[index] = result;

            file += 1;
            index += 1;
        }
    rank += 1;
    }
    attacks
}

pub fn get_pawn_attack_table(square: u8, color: Color) -> BitBoard {
    BitBoard::from_u64(
        if color == Color::White {
            WHITE_PAWN_ATTACKS[square as usize]
        }
        else {
            BLACK_PAWN_ATTACKS[square as usize]
        }
    )
}

pub fn get_knight_attack_table(square: u8) -> BitBoard {
    BitBoard::from_u64(
        KNIGHT_ATTACKS[square as usize]
    )
}

pub fn get_king_attack_table(square: u8) -> BitBoard {
    BitBoard::from_u64(
        KING_ATTACKS[square as usize]
    )
}