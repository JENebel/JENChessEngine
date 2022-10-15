use core::panic;
use std::ops::Index;

use super::*;

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
pub struct Game {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    white_occupancies: Bitboard,
    black_occupancies: Bitboard,
    all_occupancies: Bitboard,

    active_player: Color,
    enpassant_square: Square,
    castling_ability: u8,

    full_moves: u16,
    half_moves: u8
}

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
    None        = 12
}

pub const PIECE_STRINGS: [&str; 12] = ["P", "R", "N", "B", "Q", "K", "p", "r", "n", "b", "q", "k"];

impl Game {
    pub fn pretty_print(&self) {
        println!("\n  ┌────┬────┬────┬────┬────┬────┬────┬────┐");
        for y in 0..8 {
            print!("{} │", format!("{}", 8-y ).as_str());
            for x in 0..8 {
                let piece = 
                    if self.bitboards[0].get_bit(8*y+x)         { "P." }
                    else if self.bitboards[1].get_bit(8*y+x)    { "R." }
                    else if self.bitboards[2].get_bit(8*y+x)    { "N." }
                    else if self.bitboards[3].get_bit(8*y+x)    { "B." }
                    else if self.bitboards[4].get_bit(8*y+x)    { "Q." }
                    else if self.bitboards[5].get_bit(8*y+x)    { "K." }
                    else if self.bitboards[6].get_bit(8*y+x)    { "p " }
                    else if self.bitboards[7].get_bit(8*y+x)    { "r " }
                    else if self.bitboards[8].get_bit(8*y+x)    { "n " }
                    else if self.bitboards[9].get_bit(8*y+x)    { "b " }
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
        println!("   Castling:   {}\n", self.castling_ability_string());
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
        Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
    }

    pub fn new_from_fen(input: &str) -> Self {
        let fen = input.trim();
        let mut split = fen.split(' ').peekable();

        let mut bitboards =        [Bitboard::new(); 12];
        let mut white_occupancies = Bitboard::new();
        let mut black_occupancies = Bitboard::new();
        let mut all_occupancies =   Bitboard::new();

        let mut i = 0;
        let board_str = split.next().unwrap();
        for char in board_str.chars() {
            if char.is_numeric(){
                for _i in 0..char.to_digit(10).unwrap_or(0) {
                    i += 1;
                }
            }
            else if char != '/' {
                bitboards[char_to_piece(char) as usize].set_bit(i);
                all_occupancies.set_bit(i);
                if char.is_uppercase() { white_occupancies.set_bit(i) } else { black_occupancies.set_bit(i) };

                i+=1;
            }
        }

        let active = split.next().unwrap();
        let active_color = if active == "w" { Color::White } else { Color::Black };

        let castling_str =  if split.peek().is_some() { split.next().unwrap() } else { "" };
        let mut castling_ability: u8 = 0;
        if castling_str.contains('K') {castling_ability = castling_ability | CastlingAbility::WhiteKingSide as u8 }
        if castling_str.contains('Q') {castling_ability = castling_ability | CastlingAbility::WhiteQueenSide as u8}
        if castling_str.contains('k') {castling_ability = castling_ability | CastlingAbility::BlackKingSide as u8}
        if castling_str.contains('q') {castling_ability = castling_ability | CastlingAbility::BlackQueenSide as u8}

        let enpassant = if split.peek().is_some() { split.next().unwrap() } else { "-" };
        let enpassant_sq: Square = if enpassant != "-" { square_from_string(enpassant) } else { Square::None };

        let half_moves: u8 =  if split.peek().is_some() { split.next().unwrap().parse::<u8>().unwrap()  } else { 0 };
        let full_moves: u16 = if split.peek().is_some() { split.next().unwrap().parse::<u16>().unwrap() } else { 0 };

        Self { 
            bitboards: bitboards,
            white_occupancies: white_occupancies,
            black_occupancies: black_occupancies,
            all_occupancies: all_occupancies,

            active_player: active_color,
            castling_ability: castling_ability,
            enpassant_square: enpassant_sq,

            full_moves: full_moves,
            half_moves: half_moves
        }
    }

    fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
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

    pub fn print_attacked_squares (&self, by_color: Color) {
        let mut bitboard = Bitboard::new();
        for square in 0..64 {
            if self.is_square_attacked(square, by_color) { bitboard.set_bit(square) }
        }
        bitboard.print();
    }

    pub fn generate_moves(&self) -> MoveList {
        let mut moves = MoveList::new();
        let mut from_sq: u8 = 0;
        let mut to_sq: u8 = 0;

        let mut attacks;
        let mut bitboard;
        let pawn_dir: i8;
        let mut can_enpassant_attack = false;
        
        if self.active_player == Color::White { 
            bitboard = self.bitboards[0];
            pawn_dir = -8;
        } else { 
            bitboard = self.bitboards[6];
            pawn_dir = 8;
        };

        //Pawn moves
        while !bitboard.is_empty() {
            from_sq = bitboard.extract_bit();
            to_sq = (from_sq as i8 + pawn_dir) as u8;
            //Quiet and capture
            if !self.all_occupancies.get_bit(to_sq) {
                //to_sq is empty
                if to_sq >= 8 && to_sq <= 55 {
                    //Quiet moves
                    moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::None, false, false, false, false));

                    //Double push
                    to_sq = (to_sq as i8 + pawn_dir) as u8;
                    if !self.all_occupancies.get_bit(to_sq) && ((self.active_player == Color::White && from_sq / 8 == 6) || (self.active_player == Color::Black && from_sq / 8 == 1) ) {
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::None, false, true, false, false));
                    }
                }
                else {
                    //Promotions
                    if self.active_player == Color::White {
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::WhiteQueen,  false, false, false, false));
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::WhiteKnight, false, false, false, false));
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::WhiteRook,   false, false, false, false));
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::WhiteBishop, false, false, false, false))
                    } else {
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::BlackQueen,  false, false, false, false));
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::BlackKnight, false, false, false, false));
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::BlackRook,   false, false, false, false));
                        moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::None, Piece::BlackBishop, false, false, false, false))
                    }
                }
            }
            
            //Pawn attacks
            //Iniit according to color
            if self.active_player == Color::White {
                attacks = get_pawn_attack_table(from_sq, Color::White);

                can_enpassant_attack = if self.enpassant_square != Square::None {
                    !attacks.and(Bitboard::from_u64(1 << self.enpassant_square as u8)).is_empty()
                } else { false } ;

                attacks = attacks.and(self.black_occupancies)

            } else {
                attacks = get_pawn_attack_table(from_sq, Color::Black);

                can_enpassant_attack = if self.enpassant_square != Square::None { 
                    !attacks.and(Bitboard::from_u64(1 << self.enpassant_square as u8)).is_empty() 
                } else { false } ;

                attacks = attacks.and(self.white_occupancies)
            };

            //Enpassant
            if can_enpassant_attack {
                moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::WhiteKnight, Piece::None, true, false, true, false));
            }

            if from_sq >= 8 && from_sq <= 55 {
                //Normal captures
                while !attacks.is_empty() {
                    to_sq = attacks.extract_bit();
                    moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::WhiteKnight, Piece::None, true, false, false, false));
                }
            }
            else {
                //Promotions
                while !attacks.is_empty() {
                    to_sq = attacks.extract_bit();
                    moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::WhiteKnight, Piece::BlackQueen,  true, false, false, false));
                    moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::WhiteKnight, Piece::BlackKnight, true, false, false, false));
                    moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::WhiteKnight, Piece::BlackRook,   true, false, false, false));
                    moves.add_move(Move::new(SQUARES[from_sq as usize], SQUARES[to_sq as usize], Piece::WhiteKnight, Piece::BlackBishop, true, false, false, false))
                }
            }
            
        }




        moves
    }
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

pub fn char_to_piece(char: char) -> Piece {
    match char {
        'P' => Piece::WhitePawn,
        'R' => Piece::WhiteRook,
        'N' => Piece::WhiteKnight,
        'B' => Piece::WhiteBishop,
        'Q' => Piece::WhiteQueen,
        'K' => Piece::WhiteKing,
        'p' => Piece::BlackPawn,
        'r' => Piece::BlackRook,
        'n' => Piece::BlackKnight,
        'b' => Piece::BlackBishop,
        'q' => Piece::BlackQueen,
        'k' => Piece::BlackKing,
        _ => panic!("Illegal piece char: {}", char)
    }
}