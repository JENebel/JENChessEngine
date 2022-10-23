use super::*;

#[derive(Clone, Copy)]
pub struct Game {
    pub bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub white_occupancies: Bitboard,
    pub black_occupancies: Bitboard,
    pub all_occupancies: Bitboard,

    pub active_player: Color,
    pub enpassant_square: Square,
    pub castling_ability: u8,

    pub full_moves: u16,
    pub half_moves: u8
}

impl Game {
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
        Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
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

        Some(Self { 
            bitboards: bitboards,
            white_occupancies: white_occupancies,
            black_occupancies: black_occupancies,
            all_occupancies: all_occupancies,

            active_player: active_color,
            castling_ability: castling_ability,
            enpassant_square: enpassant_sq,

            full_moves: full_moves,
            half_moves: half_moves
        })
    }

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

    #[cfg(test)]
    #[allow(dead_code)]
    pub fn print_attacked_squares (&self, by_color: Color) {
        let mut bitboard = Bitboard::new();
        for square in 0..64 {
            if self.is_square_attacked(square, by_color) { bitboard.set_bit(square) }
        }
        bitboard.print();
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        if color == Color::White {
            self.is_square_attacked(self.get_piece_bitboard(Piece::WhiteKing).least_significant(), Color::Black)
        }
        else {
            self.is_square_attacked(self.get_piece_bitboard(Piece::BlackKing).least_significant(), Color::White)
        }
    }

    pub fn get_piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece as usize]
    }

    pub fn parse_move(&mut self, input: String) -> Option<Move> {
        let moves = generate_moves(&mut *self).values();
        let m = moves.iter().find(|m| m.to_uci() == input);
        match m {
            Some(a) => Some(a.clone()),
            None => None,
        }
    }
}