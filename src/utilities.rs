
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
    WhiteRook   = 1,
    WhiteKnight = 2,
    WhiteBishop = 3,
    WhiteQueen  = 4,
    WhiteKing   = 5,
    BlackPawn   = 6,
    BlackRook   = 7,
    BlackKnight = 8,
    BlackBishop = 9,
    BlackQueen  = 10,
    BlackKing   = 11,
    None        = 12,
}

pub const PIECE_STRINGS: [&str; 13] = ["P", "R", "N", "B", "Q", "K", "p", "r", "n", "b", "q", "k", "None"];

pub const MATERIAL_WEIGHTS: [i32; 12] = [100, 500, 300, 350, 1000, 10000, -100, -500, -300, -350, -1000, -10000];

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