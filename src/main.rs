mod game;
mod utility;
mod bit_board;
mod attack_tables;

use bitintr::*;
//use bitintr::*;
use game::*;
use utility::*;
use bit_board::*;
use attack_tables::*;

macro_rules! mlb {
    ($line0:tt $line1:tt $line2:tt $line3:tt $line4:tt $line5:tt $line6:tt $line7:tt) => {
        ($line0 << 56) |
        ($line1 << 48) |
        ($line2 << 40) |
        ($line3 << 32) |
        ($line4 << 24) |
        ($line5 << 16) |
        ($line6 <<  8) |
        ($line7 <<  0)
    }
}

fn main() {
    
    let n:   u64 =     mlb!(0b00100000
                            0b00000000
                            0b00001010
                            0b00000100
                            0b00100000
                            0b00000001
                            0b00000000
                            0b00000000);

    let mask1:  u64 =  mlb!(0b00000000
                            0b00010000
                            0b00001000
                            0b00000100
                            0b00000010
                            0b00000000
                            0b00000000
                            0b00000000);

    let mask2:  u64 =  mlb!(0b00000000
                            0b00000100
                            0b00001000
                            0b00010000
                            0b00100000
                            0b01000000
                            0b00000000
                            0b00000000);

    BitBoard::from_u64(n).print_bit_board();
    BitBoard::from_u64(mask1).print_bit_board();
    BitBoard::from_u64(mask2).print_bit_board();

    let res1 = n.pext(mask1);
    let res2 = n.pext(mask2);

    BitBoard::from_u64(res1.pdep(mask1)).print_bit_board();
    BitBoard::from_u64(res2.pdep(mask2)).print_bit_board();

   //println!("{}", n.popcnt());

    println!("{:#018b}", res1);
    println!("{:#018b}", res2);
    

    //get_bishop_mask(Square::d4 as u8).print_bit_board()
}