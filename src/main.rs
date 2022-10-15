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
    println!("{}", get_queen_attack_table(Square::e6 as u8, Bitboard::new()).to_u64());
    println!("{}", get_rook_attack_table(Square::e6 as u8, Bitboard::new()).to_u64());
    println!("{}", get_bishop_attack_table(Square::e6 as u8, Bitboard::new()).to_u64());
    
    //let game = Game::new_from_fen("8/8/8/pP6/8/8/8/8 w - a6 0 1");
    let game = Game::new_from_start_pos();

    game.pretty_print();

    /*let moves = game.generate_moves();

    moves.print()*/
}