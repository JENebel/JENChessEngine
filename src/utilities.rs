use std::{thread, io::stdin, sync::mpsc::{self, Receiver}};

use crate::cmove::Move;

pub struct IoWrapper {
    receiver: Receiver<String>
}

impl IoWrapper {
    pub fn init() -> Self {
        Self { receiver: init_input_thread( )}
    }

    pub fn try_read_line(&self) -> Option<String> {
        match self.receiver.try_recv() {
            Ok(line) => Some(line.trim().to_string()),
            Err(_) => None,
        }
    }

    pub fn read_line(&self) -> String {
        match self.receiver.recv() {
            Ok(line) => line.trim().to_string(),
            Err(_) => unreachable!(),
        }
    }
}

fn init_input_thread() -> Receiver<String> {
    let (tx, rx) = mpsc::channel::<String>();
    thread::spawn(move || loop {
        let mut buffer = String::new();
        stdin().read_line(&mut buffer).unwrap();
        tx.send(buffer).unwrap_or_default();
    });
    rx
}

pub fn opposite_color(color: Color) -> Color {
    if color == Color::White { Color::Black } else { Color::White }
}

pub fn square_from_string(string: &str) -> Square {
    let chars = string.as_bytes();
    let x = chars[0] - 97;
    let y = 8 - (chars[1] as char).to_digit(10).unwrap() as usize;
    SQUARES[8 * y + x as usize]
}

pub fn char_to_piece(char: char) -> Option<Piece> {
    match char {
        'P' => Some(Piece::WhitePawn),
        'R' => Some(Piece::WhiteRook),
        'N' => Some(Piece::WhiteKnight),
        'B' => Some(Piece::WhiteBishop),
        'Q' => Some(Piece::WhiteQueen),
        'K' => Some(Piece::WhiteKing),
        'p' => Some(Piece::BlackPawn),
        'r' => Some(Piece::BlackRook),
        'n' => Some(Piece::BlackKnight),
        'b' => Some(Piece::BlackBishop),
        'q' => Some(Piece::BlackQueen),
        'k' => Some(Piece::BlackKing),
        _ => None
    }
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    White
}

#[derive(Clone, Copy)]
pub enum CastlingAbility {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8
}

#[derive(Clone, Copy)]
pub enum Piece {
    WhitePawn   = 0,
    WhiteKnight = 1,
    WhiteBishop = 2,
    WhiteRook   = 3,
    WhiteQueen  = 4,
    WhiteKing   = 5,
    BlackPawn   = 6,
    BlackKnight = 7,
    BlackBishop = 8,
    BlackRook   = 9,
    BlackQueen  = 10,
    BlackKing   = 11,
    None        = 12,
}

pub const PIECE_STRINGS: [&str; 13] = ["P", "N", "B", "R", "Q", "K", "p", "n", "b", "r", "q", "k", "None"];

pub const CASTLING_RIGHTS: [u8; 64] = [
    7, 15, 15, 15,  3, 15, 15, 11,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   15, 15, 15, 15, 15, 15, 15, 15,
   13, 15, 15, 15, 12, 15, 15, 14
];

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

pub const MATERIAL_WEIGHTS: [i32; 12] = [100, 300, 350, 500, 1000, 10000, -100, -300, -350, -500, -1000, -10000];

// pawn positional score
pub const PAWN_SCORES: [i32; 64] = 
[
    90,  90,  90,  90,  90,  90,  90,  90,
    30,  30,  30,  40,  40,  30,  30,  30,
    20,  20,  20,  30,  30,  30,  20,  20,
    10,  10,  10,  20,  20,  10,  10,  10,
     5,   5,  10,  20,  20,   5,   5,   5,
     0,   0,   0,   5,   5,   0,   0,   0,
     0,   0,   0, -10, -10,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0
];

// knight positional score
pub const KNIGHT_SCORES: [i32; 64] = 
[
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5,   0,   0,  10,  10,   0,   0,  -5,
    -5,   5,  20,  20,  20,  20,   5,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,  10,  20,  30,  30,  20,  10,  -5,
    -5,   5,  20,  10,  10,  20,   5,  -5,
    -5,   0,   0,   0,   0,   0,   0,  -5,
    -5, -10,   0,   0,   0,   0, -10,  -5
];

// bishop positional score
pub const BISHOP_SCORES: [i32; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   0,  10,  10,   0,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,  10,   0,   0,   0,   0,  10,   0,
     0,  30,   0,   0,   0,   0,  30,   0,
     0,   0, -10,   0,   0, -10,   0,   0

];

// rook positional score
pub const ROOK_SCORES: [i32; 64] = 
[
    50,  50,  50,  50,  50,  50,  50,  50,
    50,  50,  50,  50,  50,  50,  50,  50,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,  10,  20,  20,  10,   0,   0,
     0,   0,   0,  20,  20,   0,   0,   0

];

// king positional score
pub const KING_SCORES: [i32; 64] = 
[
     0,   0,   0,   0,   0,   0,   0,   0,
     0,   0,   5,   5,   5,   5,   0,   0,
     0,   5,   5,  10,  10,   5,   5,   0,
     0,   5,  10,  20,  20,  10,   5,   0,
     0,   5,  10,  20,  20,  10,   5,   0,
     0,   0,   5,  10,  10,   5,   0,   0,
     0,   5,   5,  -5,  -5,   0,   5,   0,
     0,   0,   5,   0, -15,   0,  10,   0
];

// mirror positional score tables for opposite side
pub const MIRRORED: [usize; 64] = 
[
	56, 57, 58, 59, 60, 61, 62, 63,
	48, 49, 50, 51, 52, 53, 54, 55,
	40, 41, 42, 43, 44, 45, 46, 47,
	32, 33, 34, 35, 36, 37, 38, 39,
	24, 25, 26, 27, 28, 29, 30, 31,
	16, 17, 18, 19, 20, 21, 22, 23,
	8,  9,  10, 11, 12, 13, 14, 15,
	0,  1,  2,  3,  4,  5,  6,  7
];

///[attacker][victim]
pub const MVV_LVA: [[i32; 12]; 12] = [
    [105, 205, 305, 405, 505, 605,  105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604,  104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603,  103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602,  102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601,  101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600,  100, 200, 300, 400, 500, 600],
    
    [105, 205, 305, 405, 505, 605,  105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604,  104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603,  103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602,  102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601,  101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600,  100, 200, 300, 400, 500, 600],
];

pub const PIECE_KEYS: [[u64; 64]; 12] = generate_piece_keys();
pub const ENPASSANT_KEYS: [u64; 64] = generate_enpassant_keys();
pub const CASTLE_KEYS: [u64; 16] = generate_castle_keys();
pub const SIDE_KEY: u64 = get_random_u64_number(4084590338).0;

const fn generate_castle_keys() -> [u64; 16] {
    let mut keys = [0; 16];

    let mut i = 0;
    let mut state = 3667794840;

    while i < 16 {
        let res = get_random_u64_number(state);
        state = res.1;
        keys[i] = res.0;
        i+=1;
    }

    keys
}

const fn generate_enpassant_keys() -> [u64; 64] {
    let mut keys = [0; 64];

    let mut sq = 0;
    let mut state = 862131765;

    while sq < 64 {
        let res = get_random_u64_number(state);
        state = res.1;
        keys[sq] = res.0;
        sq+=1;
    }

    keys
}

const fn generate_piece_keys() -> [[u64; 64]; 12] {
    let mut keys = [[0; 64]; 12];

    let mut p  = 0;
    let mut sq;
    let mut state = 2828886037;

    while p < 12 {
        sq = 0;
        while sq < 64 {
            let res = get_random_u64_number(state);
            state = res.1;
            keys[p][sq] = res.0;
            sq+=1;
        }
        p+=1
    }

    keys
}

const fn get_random_u32_number(state: u32) -> u32{
    let mut num: u64 = state as u64;

    // XOR shift algorithm
    num ^= num << 13;
    num ^= num >> 17;
    num ^= num << 5;

    // return random number
    return num as u32;
}

// generate 64-bit pseudo legal numbers
const fn get_random_u64_number(state: u32) -> (u64, u32) {
    // define 4 random numbers
    let n1 = get_random_u32_number(state);
    let n2 = get_random_u32_number(n1);
    let n3 = get_random_u32_number(n2);
    let n4 = get_random_u32_number(n3);
    
    // return random number
    return (n1 as u64 | ((n2 as u64) << 16) | ((n3 as u64) << 32) | ((n4 as u64) << 48), n4);
}

pub struct SearchResult {
    pub best_move: Move,
    pub nodes_visited: u64,
    pub score: i32,
    pub depth: u8,
    pub reached_max_ply: bool,
    pub tt_hits: u32
}

impl SearchResult {
    pub fn new(cmove: Move, nodes: u64, score: i32, depth: u8, reached_max_ply: bool, tt_hits: u32) -> Self {
        Self { best_move: cmove, nodes_visited: nodes, score: score, depth: depth, reached_max_ply: reached_max_ply, tt_hits: tt_hits }
    }
}