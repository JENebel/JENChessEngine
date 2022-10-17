#![feature(const_eval_limit)]
#![const_eval_limit = "0"]

mod game;
mod bitboard;
mod attack_tables;
mod attack_tables_ctg;
mod cmove;
mod move_list;

use game::*;
use cmove::*;
use bitboard::*;
use attack_tables::*;
use move_list::*;

fn main() {
    let mut game = Game::new_from_fen("r4rk1/1pp1qppp/p1np1n2/2b1p1B1/2B1P1b1/P1NP1N2/1PP1QPPP/R4RK1 w - - 0 10 ");
    
    game.pretty_print();
    let res = game.TEMP_PERFT(5, true);

    println!("{res}");
}