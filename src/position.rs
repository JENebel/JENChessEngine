use super::*;

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

pub fn square_from_string(string: &str) -> Square {
    let chars = string.as_bytes();
    let x = chars[0] - 97;
    let y = 8 - (chars[1] as char).to_digit(10).unwrap() as usize;
    SQUARES[8 * y + x as usize]
}

#[derive(Clone, Copy)]
pub enum CastlingAbility {
    WhiteKingSide = 1,
    WhiteQueenSide = 2,
    BlackKingSide = 4,
    BlackQueenSide = 8
}

#[derive(Debug)]
#[derive(Clone, Copy)]
#[derive(PartialEq)]
pub enum Color {
    Black,
    White
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

pub fn opposite_color(color: Color) -> Color {
    if color == Color::White { Color::Black } else { Color::White }
}

#[derive(Clone, Copy)]
pub struct Position {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub white_occupancies: Bitboard,
    pub black_occupancies: Bitboard,
    pub all_occupancies: Bitboard,

    pub active_player: Color,
    pub enpassant_square: Square,
    pub castling_ability: u8,

    pub full_moves: u16,
    pub half_moves: u8,
    pub zobrist_hash: u64,
}

impl Position {
    pub fn pretty_print(&self) {
        println!("\n  ┌────┬────┬────┬────┬────┬────┬────┬────┐");
        for y in 0..8 {
            print!("{} │", format!("{}", 8-y ).as_str());
            for x in 0..8 {
                let piece = 
                    if self.bitboards[0].get_bit(8*y+x)         { "P." }
                    else if self.bitboards[1].get_bit(8*y+x)    { "N." }
                    else if self.bitboards[2].get_bit(8*y+x)    { "B." }
                    else if self.bitboards[3].get_bit(8*y+x)    { "R." }
                    else if self.bitboards[4].get_bit(8*y+x)    { "Q." }
                    else if self.bitboards[5].get_bit(8*y+x)    { "K." }
                    else if self.bitboards[6].get_bit(8*y+x)    { "p " }
                    else if self.bitboards[7].get_bit(8*y+x)    { "n " }
                    else if self.bitboards[8].get_bit(8*y+x)    { "b " }
                    else if self.bitboards[9].get_bit(8*y+x)    { "r " }
                    else if self.bitboards[10].get_bit(8*y+x)   { "q " }
                    else if self.bitboards[11].get_bit(8*y+x)   { "k " }
                    else { "  " };
                print!(" {piece} ");

                if x != 7 { print!("│") };
            }
            println!("│");
            if y != 7 { println!("  ├────┼────┼────┼────┼────┼────┼────┼────┤")};
        }
        println!("  └────┴────┴────┴────┴────┴────┴────┴────┘");
        println!("    a    b    c    d    e    f    g    h\n");

        print!("   Active:     {}",     if self.active_player == Color::White { "White" } else { "Black" });
        println!("\tFull moves: {}",    self.full_moves);
        print!("   Enpassant:  {}",     SQUARE_STRINGS[self.enpassant_square as usize]);
        println!("\tHalf moves: {}",    self.half_moves);
        print!("   Castling:   {}  ", self.castling_ability_string());
        println!("\tZobrist:   {:#0x}\n", self.make_zobrist_hash());
    }

    fn castling_ability_string(&self) -> String {
        let mut result = String::new();
        if self.castling_ability & CastlingAbility::WhiteKingSide as u8 != 0    { result += "K" }
        if self.castling_ability & CastlingAbility::WhiteQueenSide as u8 != 0   { result += "Q" }
        if self.castling_ability & CastlingAbility::BlackKingSide as u8 != 0    { result += "k" }
        if self.castling_ability & CastlingAbility::BlackQueenSide as u8 != 0   { result += "q" }
        result
    }

    pub fn new_from_start_pos() -> Self {
        Position::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn new_from_fen(input: &str) -> Option<Self> {
        let fen = input.trim();
        let mut split = fen.split(' ').peekable();

        let mut bitboards =        [Bitboard::new(); 12];
        let mut white_occupancies = Bitboard::new();
        let mut black_occupancies = Bitboard::new();
        let mut all_occupancies =   Bitboard::new();

        let mut i = 0;

        if split.peek().is_none() { return None }
        let board_str = split.next().unwrap();

        for char in board_str.chars() {
            if char.is_numeric(){
                for _i in 0..char.to_digit(10).unwrap_or(0) {
                    i += 1;
                }
            }
            else if char != '/' {
                let piece = char_to_piece(char);
                if piece.is_none() { return None };
                bitboards[char_to_piece(char).unwrap() as usize].set_bit(i);
                all_occupancies.set_bit(i);
                if char.is_uppercase() { white_occupancies.set_bit(i) } else { black_occupancies.set_bit(i) };

                i+=1;
            }
        }

        if split.peek().is_none() { return None }
        let active_str = split.next().unwrap();
        let active_player = if active_str == "w" { Color::White } else { Color::Black };

        let castling_str =  if split.peek().is_some() { split.next().unwrap() } else { "" };
        let mut castling_ability: u8 = 0;
        if castling_str.contains('K') {castling_ability = castling_ability | CastlingAbility::WhiteKingSide as u8 }
        if castling_str.contains('Q') {castling_ability = castling_ability | CastlingAbility::WhiteQueenSide as u8}
        if castling_str.contains('k') {castling_ability = castling_ability | CastlingAbility::BlackKingSide as u8}
        if castling_str.contains('q') {castling_ability = castling_ability | CastlingAbility::BlackQueenSide as u8}

        let enpassant_str = if split.peek().is_some() { split.next().unwrap() } else { "-" };
        let enpassant_square: Square = if enpassant_str != "-" { square_from_string(enpassant_str) } else { Square::None };

        let half_moves: u8 =  if split.peek().is_some() { split.next().unwrap().parse::<u8>().unwrap()  } else { 0 };
        let full_moves: u16 = if split.peek().is_some() { split.next().unwrap().parse::<u16>().unwrap() } else { 0 };

        let mut pos = Self { 
            bitboards,
            white_occupancies,
            black_occupancies,
            all_occupancies,

            active_player,
            castling_ability,
            enpassant_square,

            full_moves,
            half_moves,
            zobrist_hash: 0
        };

        pos.zobrist_hash = pos.make_zobrist_hash();

        Some(pos)
    }

    #[inline(always)]
    pub fn get_piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece as usize]
    }

    ///Creates a zobrist hash from scratch for the current position
    pub fn make_zobrist_hash(&self) -> u64 {
        let mut hash = 0;

        for piece in 0..12 {
            let mut bb = self.bitboards[piece];
            while !bb.is_empty() {
                let ind = bb.extract_bit();

                hash ^= PIECE_KEYS[piece][ind as usize];
            }
        }

        hash ^= CASTLE_KEYS[self.castling_ability as usize];
        
        if self.active_player == Color::Black {
            hash ^= SIDE_KEY;
        }

        if self.enpassant_square != Square::None {
            hash ^= ENPASSANT_KEYS[self.enpassant_square as usize];
        }

        hash
    }

    #[inline(always)]
    ///Indicates whether a square is attacked
    pub fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
        return if by_color == Color::White {
            !get_pawn_attack_table(square, Color::Black).and(self.bitboards[Piece::WhitePawn as usize]).is_empty() ||
            !get_knight_attack_table(square).and(self.bitboards[Piece::WhiteKnight as usize]).is_empty() ||
            !get_king_attack_table(square).and(self.bitboards[Piece::WhiteKing as usize]).is_empty() ||
            !get_rook_attack_table(square, self.all_occupancies).and(self.bitboards[Piece::WhiteRook as usize]).is_empty() ||
            !get_bishop_attack_table(square, self.all_occupancies).and(self.bitboards[Piece::WhiteBishop as usize]).is_empty() ||
            !get_queen_attack_table(square, self.all_occupancies).and(self.bitboards[Piece::WhiteQueen as usize]).is_empty() 
        }
        else {
            !get_pawn_attack_table(square, Color::White).and(self.bitboards[Piece::BlackPawn as usize]).is_empty() ||
            !get_knight_attack_table(square).and(self.bitboards[Piece::BlackKnight as usize]).is_empty() ||
            !get_king_attack_table(square).and(self.bitboards[Piece::BlackKing as usize]).is_empty() ||
            !get_rook_attack_table(square, self.all_occupancies).and(self.bitboards[Piece::BlackRook as usize]).is_empty() ||
            !get_bishop_attack_table(square, self.all_occupancies).and(self.bitboards[Piece::BlackBishop as usize]).is_empty() ||
            !get_queen_attack_table(square, self.all_occupancies).and(self.bitboards[Piece::BlackQueen as usize]).is_empty() 
        }
    }

    ///Indicates whether the given color's king is in check
    #[inline(always)]
    pub fn is_in_check(&self, color: Color) -> bool {
        if color == Color::White {
            self.is_square_attacked(self.get_piece_bitboard(Piece::WhiteKing).least_significant(), Color::Black)
        }
        else {
            self.is_square_attacked(self.get_piece_bitboard(Piece::BlackKing).least_significant(), Color::White)
        }
    }
}