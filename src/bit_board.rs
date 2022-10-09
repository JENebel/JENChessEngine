use bitintr::Pext;

use crate::*;

#[derive(Copy, Clone)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
pub enum Square {
    a8,  b8,  c8,  d8,  e8,  f8,  g8,  h8,
    a7,  b7,  c7,  d7,  e7,  f7,  g7,  h7,
    a6,  b6,  c6,  d6,  e6,  f6,  g6,  h6,
    a5,  b5,  c5,  d5,  e5,  f5,  g5,  h5,
    a4,  b4,  c4,  d4,  e4,  f4,  g4,  h4,
    a3,  b3,  c3,  d3,  e3,  f3,  g3,  h3,
    a2,  b2,  c2,  d2,  e2,  f2,  g2,  h2,
    a1,  b1,  c1,  d1,  e1,  f1,  g1,  h1
}

#[derive(Clone, Copy)]
pub struct BitBoard {
    bits: u64,
}

impl BitBoard {
    pub fn from_u64(source: u64) -> Self{
        Self { bits: source }
    }

    pub fn new() -> Self{
        Self { bits: 0 }
    }

    pub fn print_bit_board (&self) {
        println!();
        for rank in 0..8 {
            print!("{}  ", 8 - rank);  
    
            for file in 0..8 {
                print!( " {} ", if self.get_bit(rank*8 + file) { "X" } else { "." } )
            }
            println!()
        }
        println!( "    a  b  c  d  e  f  g  h");
        println!( "     Bitboard: {}", self.bits)
    }
    
    pub fn get_bit(&self, square: u8) -> bool {
        self.bits & (1 << square) != 0
    }
    
    pub fn get_bit_sq(&self, square: Square) -> bool {
        self.get_bit(square as u8)
    }
    
    pub fn set_bit(&mut self, square: u8) {
        self.bits |= 1 << square
    }

    pub fn set_bit_sq(&mut self, square: Square) {
        self.set_bit(square as u8)
    }

    pub fn unset_bit(&mut self, square: u8) {
        self.bits &= (1 << square) ^ self.bits
    }

    pub fn unset_bit_sq(&mut self, square: Square) {
        self.unset_bit(square as u8)
    }
}