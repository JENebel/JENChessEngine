use bitintr::{Pext, Blsr};

use crate::*;

#[derive(Copy, Clone)]
#[derive(PartialEq)]
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
    a1,  b1,  c1,  d1,  e1,  f1,  g1,  h1, 
    None
}

pub const SQUARE_STRINGS: [&str; 65] = [
    "a8",  "b8",  "c8",  "d8",  "e8",  "f8",  "g8",  "h8",
    "a7",  "b7",  "c7",  "d7",  "e7",  "f7",  "g7",  "h7",
    "a6",  "b6",  "c6",  "d6",  "e6",  "f6",  "g6",  "h6",
    "a5",  "b5",  "c5",  "d5",  "e5",  "f5",  "g5",  "h5",
    "a4",  "b4",  "c4",  "d4",  "e4",  "f4",  "g4",  "h4",
    "a3",  "b3",  "c3",  "d3",  "e3",  "f3",  "g3",  "h3",
    "a2",  "b2",  "c2",  "d2",  "e2",  "f2",  "g2",  "h2",
    "a1",  "b1",  "c1",  "d1",  "e1",  "f1",  "g1",  "h1", 
    "None"
];

pub const SQUARES: [Square; 65] = [
    Square::a8,  Square::b8,  Square::c8,  Square::d8,  Square::e8,  Square::f8,  Square::g8,  Square::h8,
    Square::a7,  Square::b7,  Square::c7,  Square::d7,  Square::e7,  Square::f7,  Square::g7,  Square::h7,
    Square::a6,  Square::b6,  Square::c6,  Square::d6,  Square::e6,  Square::f6,  Square::g6,  Square::h6,
    Square::a5,  Square::b5,  Square::c5,  Square::d5,  Square::e5,  Square::f5,  Square::g5,  Square::h5,
    Square::a4,  Square::b4,  Square::c4,  Square::d4,  Square::e4,  Square::f4,  Square::g4,  Square::h4,
    Square::a3,  Square::b3,  Square::c3,  Square::d3,  Square::e3,  Square::f3,  Square::g3,  Square::h3,
    Square::a2,  Square::b2,  Square::c2,  Square::d2,  Square::e2,  Square::f2,  Square::g2,  Square::h2,
    Square::a1,  Square::b1,  Square::c1,  Square::d1,  Square::e1,  Square::f1,  Square::g1,  Square::h1, 
    Square::None
];

#[derive(Clone, Copy)]
pub struct Bitboard {
    bits: u64,
}

impl Bitboard {
    pub fn from_u64(source: u64) -> Self {
        Self { bits: source }
    }

    pub fn new() -> Self {
        Self { bits: 0 }
    }

    pub fn to_u64(&self) -> u64 {
        self.bits
    }

    //#[inline(always)]
    pub fn print (&self) {
        println!();
        for rank in 0..8 {
            print!("{}  ", 8 - rank);  
    
            for file in 0..8 {
                print!( " {} ", if self.get_bit(rank*8 + file) { "X" } else { "-" } )
            }
            println!()
        }
        println!( "    a  b  c  d  e  f  g  h");
        println!( "    Bitboard: {}", self.bits)
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

    pub fn is_empty(&self) -> bool {
        self.bits == 0
    }

    pub fn least_significant(&self) -> u8 {
        self.bits.trailing_zeros() as u8
    }

    ///Extract the least significant 1-bit. Modifies the bitboard and returns the position of the extracted bit
    pub fn extract_bit(&mut self) -> u8 {
        let last1 = self.bits.trailing_zeros();

        self.bits = self.bits.blsr();

        last1 as u8
    }

    pub fn and(&self, other: Bitboard) -> Self {
        Self { bits: self.bits & other.bits }
    }

    pub fn or(&self, other: Bitboard) -> Self {
        Self { bits: self.bits | other.bits }
    }

    pub fn xor(&self, other: Bitboard) -> Self {
        Self { bits: self.bits ^ other.bits }
    }
}

pub fn not(bitboard: Bitboard) -> Bitboard {
    Bitboard { bits: !bitboard.bits }
}