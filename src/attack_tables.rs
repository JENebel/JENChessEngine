use bitintr::{Pdep, Pext};

use super::consts::*;

use crate::{game::Color, bitboard::Bitboard};

//Getters
pub fn get_pawn_attack_table(square: u8, color: Color) -> Bitboard {
    Bitboard::from_u64(
        if color == Color::White {
            WHITE_PAWN_ATTACKS[square as usize]
        }
        else {
            BLACK_PAWN_ATTACKS[square as usize]
        }
    )
}

pub fn get_knight_attack_table(square: u8) -> Bitboard {
    Bitboard::from_u64(
        KNIGHT_ATTACKS[square as usize]
    )
}

pub fn get_king_attack_table(square: u8) -> Bitboard {
    Bitboard::from_u64(
        KING_ATTACKS[square as usize]
    )
}

pub fn get_rook_attack_table(square: u8, occ: Bitboard) -> Bitboard {
    let attacks = SLIDING_ATTACKS[(ROOK_OFFSETS[square as usize] as u64 + occ.to_u64().pext(ROOK_MASK[square as usize])) as usize];
    Bitboard::from_u64(
        (attacks as u64).pdep(ROOK_ATTACK_MASK[square as usize])
    )
}

pub fn get_bishop_attack_table(square: u8, occ: Bitboard) -> Bitboard {
    let attacks = SLIDING_ATTACKS[(BISHOP_OFFSETS[square as usize] as u64 + occ.to_u64().pext(BISHOP_MASK[square as usize])) as usize];
    Bitboard::from_u64(
        (attacks as u64).pdep(BISHOP_ATTACK_MASK[square as usize])
    )
}

pub fn get_queen_attack_table(square: u8, occ: Bitboard) -> Bitboard {
    let bishop_raw = SLIDING_ATTACKS[(BISHOP_OFFSETS[square as usize] as u64 + occ.to_u64().pext(BISHOP_MASK[square as usize])) as usize];
    let rook_raw = SLIDING_ATTACKS[(ROOK_OFFSETS[square as usize] as u64 + occ.to_u64().pext(ROOK_MASK[square as usize])) as usize];

    let rook_attacks = (rook_raw as u64).pdep(ROOK_ATTACK_MASK[square as usize]);
    let bishop_attacks = (bishop_raw as u64).pdep(BISHOP_ATTACK_MASK[square as usize]);
    
    Bitboard::from_u64 (
        rook_attacks | bishop_attacks
    )
}