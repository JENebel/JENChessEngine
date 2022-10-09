mod game;
mod utility;
mod bit_board;
mod attack_tables;

use bitintr::{Pext, Popcnt};
use game::*;
use utility::*;
use bit_board::*;
use attack_tables::*;

fn main() {
    let n:      u64 =     0b00100000_00000000_00000010_00000100_00000000_00000001_00000000_00000000;
    let mask:   u64 =     0b00100000_00010001_00001010_00000100_00001010_00010001_00100000_01000000;

    let res = n.pext(mask);

    println!("{}", n.popcnt());
    println!("{:#018b}", res)
}