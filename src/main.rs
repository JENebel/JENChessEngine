#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

mod game;
mod bitboard;
mod attack_tables;
mod attack_tables_ctg;
mod cmove;

use game::*;
use bitboard::*;
use attack_tables::*;
use attack_tables_ctg as consts;

fn main() {
    let mut occ = Bitboard::new();
    occ.set_bit_sq(Square::f6);
    occ.set_bit_sq(Square::e4);



    println!("{}", get_queen_attack_table(Square::e6 as u8, occ).to_u64());
    println!("{}", get_rook_attack_table(Square::e6 as u8, occ).to_u64());
    println!("{}", get_bishop_attack_table(Square::e6 as u8, occ).to_u64());
}