#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

mod game;
mod bit_board;
mod constants;
mod attack_tables;

use game::*;
use bit_board::*;
use constants::*;
use attack_tables::*;

fn main() {
    let mut occ = BitBoard::new();
    occ.set_bit_sq(Square::c2);
    occ.set_bit_sq(Square::e4);

    get_rook_attack_table(Square::e2 as u8, occ).print_bit_board();

    get_bishop_attack_table(Square::d5 as u8, occ).print_bit_board();
}