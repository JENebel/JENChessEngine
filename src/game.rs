use super::*;
use rayon::prelude::*;
use rand::Rng;

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

    fn get_piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece as usize]
    }

    pub fn generate_moves(&mut self) -> MoveList {
        let mut moves = MoveList::new();
        let mut from_sq: u8;
        let mut to_sq:   u8;

        let mut attacks: Bitboard;
        let mut quiet: Bitboard;

        let mut pawn_bitboard;
        let mut rook_bitboard;
        let mut knight_bitboard;
        let mut bishop_bitboard;
        let mut queen_bitboard;
        let mut king_bitboard;

        let opponent_occupancies: Bitboard;

        let rook;
        let knight;
        let bishop;
        let queen;
        let king;

        //Color specific
        //WHITE
        if self.active_player == Color::White {
            opponent_occupancies = self.black_occupancies;
            pawn_bitboard =     self.get_piece_bitboard(Piece::WhitePawn);
            rook_bitboard =     self.get_piece_bitboard(Piece::WhiteRook);
            knight_bitboard =   self.get_piece_bitboard(Piece::WhiteKnight);
            bishop_bitboard =   self.get_piece_bitboard(Piece::WhiteBishop);
            queen_bitboard =    self.get_piece_bitboard(Piece::WhiteQueen);
            king_bitboard =     self.get_piece_bitboard(Piece::WhiteKing);

            rook =   Piece::WhiteRook as u8;
            knight = Piece::WhiteKnight as u8;
            bishop = Piece::WhiteBishop as u8;
            queen =  Piece::WhiteQueen as u8;
            king =   Piece::WhiteKing as u8;

            //Pawn moves
            while !pawn_bitboard.is_empty() {
                from_sq = pawn_bitboard.extract_bit();
                to_sq = (from_sq as i8 - 8) as u8;
                //Quiet
                if !self.all_occupancies.get_bit(to_sq) {
                    //to_sq is empty
                    if to_sq >= 8 {
                        //Quiet moves
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::None as u8, false, false, false, false));
    
                        //Double push
                        to_sq = (to_sq as i8 - 8) as u8;
                        if !self.all_occupancies.get_bit(to_sq) && from_sq / 8 == 6 {
                            self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::None as u8, false, true, false, false));
                        }
                    }
                    //Promotions
                    else {
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteQueen as u8,  false, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteKnight as u8, false, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteRook as u8,   false, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteBishop as u8, false, false, false, false))
                    }
                }

                //Captures
                attacks = get_pawn_attack_table(from_sq, Color::White);
    
                //enpassant
                if self.enpassant_square != Square::None && !attacks.and(Bitboard::from_u64(1 << self.enpassant_square as u8)).is_empty(){
                    self.add_move_if_legal(&mut moves, Move::new(from_sq, self.enpassant_square as u8, Piece::WhitePawn as u8, Piece::None as u8, false, false, true, false));
                }

                //Overlap with opponent occupancies
                attacks = attacks.and(self.black_occupancies);

                while !attacks.is_empty() {
                    to_sq = attacks.extract_bit();
                    //Regular captures
                    if to_sq >= 8 {
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::None as u8, true, false, false, false));

                    //Promotions
                    } else {
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteQueen as u8,  true, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteKnight as u8, true, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteRook as u8,   true, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::WhitePawn as u8, Piece::WhiteBishop as u8, true, false, false, false))
                    }
                }
            }

            //Castling kingside
            if  self.castling_ability & (CastlingAbility::WhiteKingSide as u8) != 0 &&              //castling ability
                (self.all_occupancies.and(Bitboard::from_u64(6917529027641081856))).is_empty() &&   //f1 and g1 are free. 6917529027641081856 is f1 and g1 set
                !self.is_square_attacked(Square::e1 as u8, Color::Black) &&                         //e1 is notunder attack
                !self.is_square_attacked(Square::f1 as u8, Color::Black) {                          //f1 is not under attack

                    self.add_move_if_legal(&mut moves, Move::new(Square::e1 as u8, Square::g1 as u8, Piece::WhiteKing as u8, Piece::None as u8, false, false, false, true))
            }
            //Castling queen
            if  self.castling_ability & (CastlingAbility::WhiteQueenSide as u8) != 0 &&             //castling ability
                (self.all_occupancies.and(Bitboard::from_u64(1008806316530991104))).is_empty() &&   //d1, c1 and b1 are free. 1008806316530991104 is f1 and g1 set
                !self.is_square_attacked(Square::e1 as u8, Color::Black) &&                         //e1 is notunder attack
                !self.is_square_attacked(Square::d1 as u8, Color::Black) {                          //d1 is not under attack

                    self.add_move_if_legal(&mut moves, Move::new(Square::e1 as u8, Square::c1 as u8, Piece::WhiteKing as u8, Piece::None as u8, false, false, false, true))
            }
        }
        //BLACK
        else {
            opponent_occupancies = self.white_occupancies;
            pawn_bitboard = self.get_piece_bitboard(Piece::BlackPawn);
            rook_bitboard = self.get_piece_bitboard(Piece::BlackRook);
            knight_bitboard = self.get_piece_bitboard(Piece::BlackKnight);
            bishop_bitboard = self.get_piece_bitboard(Piece::BlackBishop);
            queen_bitboard = self.get_piece_bitboard(Piece::BlackQueen);
            king_bitboard = self.get_piece_bitboard(Piece::BlackKing);

            rook =   Piece::BlackRook as u8;
            knight = Piece::BlackKnight as u8;
            bishop = Piece::BlackBishop as u8;
            queen =  Piece::BlackQueen as u8;
            king =   Piece::BlackKing as u8;

            //Pawn moves
            while !pawn_bitboard.is_empty() {
                from_sq = pawn_bitboard.extract_bit();
                to_sq = (from_sq as i8 + 8) as u8;
                //Quiet
                if !self.all_occupancies.get_bit(to_sq) {
                    //to_sq is empty
                    if to_sq <= 55 {
                        //Quiet moves
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::None as u8, false, false, false, false));
    
                        //Double push
                        to_sq = (to_sq as i8 + 8) as u8;
                        if !self.all_occupancies.get_bit(to_sq) && from_sq / 8 == 1 {
                            self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::None as u8, false, true, false, false));
                        }
                    }
                    //Promotions
                    else {
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackQueen as u8,  false, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackKnight as u8, false, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackRook as u8,   false, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackBishop as u8, false, false, false, false))
                    }
                }

                //Captures
                attacks = get_pawn_attack_table(from_sq, Color::Black);
    
                //enpassant
                if self.enpassant_square != Square::None && !attacks.and(Bitboard::from_u64(1 << self.enpassant_square as u8)).is_empty(){
                    self.add_move_if_legal(&mut moves, Move::new(from_sq, self.enpassant_square  as u8, Piece::BlackPawn as u8, Piece::None as u8, false, false, true, false));
                }

                //Overlap with opponent occupancies
                attacks = attacks.and(self.white_occupancies);

                while !attacks.is_empty() {
                    to_sq = attacks.extract_bit();
                    //Regular captures
                    if to_sq <= 55 {
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::None as u8, true, false, false, false));

                    //Promotions
                    } else {
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackQueen as u8,  true, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackKnight as u8, true, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackRook as u8,   true, false, false, false));
                        self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, Piece::BlackPawn as u8, Piece::BlackBishop as u8, true, false, false, false))
                    }
                }
            }

            //Castling kingside
            if  self.castling_ability & (CastlingAbility::BlackKingSide as u8) != 0 &&              //castling ability
                (self.all_occupancies.and(Bitboard::from_u64(96))).is_empty() &&                    //f8 and g8 are free. 96 is f1 and g1 set
                !self.is_square_attacked(Square::e8 as u8, Color::White) &&                         //e8 is notunder attack
                !self.is_square_attacked(Square::f8 as u8, Color::White) {                          //f8 is not under attack

                    self.add_move_if_legal(&mut moves, Move::new(Square::e8 as u8, Square::g8 as u8, Piece::BlackKing as u8, Piece::None as u8, false, false, false, true))
            }
            //Castling queen
            if  self.castling_ability & (CastlingAbility::BlackQueenSide as u8) != 0 &&             //castling ability
                (self.all_occupancies.and(Bitboard::from_u64(14))).is_empty() &&                    //d8, c8 and b8 are free. 14 is f1 and g1 set
                !self.is_square_attacked(Square::e8 as u8, Color::White) &&                         //e8 is notunder attack
                !self.is_square_attacked(Square::d8 as u8, Color::White) {                          //d8 is not under attack

                    self.add_move_if_legal(&mut moves, Move::new(Square::e8 as u8, Square::c8 as u8, Piece::BlackKing as u8, Piece::None as u8, false, false, false, true))
            }
        }

        //Knight attacks
        while !knight_bitboard.is_empty() {
            from_sq = knight_bitboard.extract_bit();

            //Raw attack table
            attacks = get_knight_attack_table(from_sq);

            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(self.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, knight, Piece::None as u8, false, false, false, false))
            }

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, knight, Piece::None as u8, true, false, false, false))
            }
        }

        //Bishop attacks
        while !bishop_bitboard.is_empty() {
            from_sq = bishop_bitboard.extract_bit();

            //Raw attack table
            attacks = get_bishop_attack_table(from_sq, self.all_occupancies);

            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(self.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, bishop, Piece::None as u8, false, false, false, false))
            }

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, bishop, Piece::None as u8, true, false, false, false))
            }
        }

        //Rook attacks
        while !rook_bitboard.is_empty() {
            from_sq = rook_bitboard.extract_bit();

            //Raw attack table
            attacks = get_rook_attack_table(from_sq, self.all_occupancies);

            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(self.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, rook, Piece::None as u8, false, false, false, false))
            }

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, rook, Piece::None as u8, true, false, false, false))
            }
        }

        //Queen attacks
        while !queen_bitboard.is_empty() {
            from_sq = queen_bitboard.extract_bit();

            //Raw attack table
            attacks = get_queen_attack_table(from_sq, self.all_occupancies);

            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(self.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, queen, Piece::None as u8, false, false, false, false))
            }

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, queen, Piece::None as u8, true, false, false, false))
            }
        }

        //Queen attacks
        while !king_bitboard.is_empty() {
            from_sq = king_bitboard.extract_bit();

            //Raw attack table
            attacks = get_king_attack_table(from_sq,);

            //Extract only quiet moves and loop over them
            quiet = attacks.and(not(self.all_occupancies));
            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, king, Piece::None as u8, false, false, false, false))
            }

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.add_move_if_legal(&mut moves, Move::new(from_sq, to_sq, king, Piece::None as u8, true, false, false, false))
            }
        }

        moves
    }

    pub fn make_move(&mut self, cmove: &Move) {
        let from_square = cmove.from_square();
        let to_square   = cmove.to_square();
        let promotion   = cmove.promotion();
        let piece       = cmove.piece();
        let capturing   = cmove.is_capture();
        let double_push = cmove.is_double_push();
        let enpassant   = cmove.is_enpassant();
        let castling    = cmove.is_castling();

        //Increment half moves counter if quet and reset if pawn
        if piece == Piece::WhitePawn as u8 || piece == Piece::BlackPawn as u8 {
            self.half_moves = 0;
        }
        else {
            self.half_moves += 1;
        }

        //Update bitboards
        self.bitboards[piece as usize].unset_bit(from_square);
        self.bitboards[piece as usize].set_bit(to_square);

        self.all_occupancies.unset_bit(from_square);
        self.all_occupancies.set_bit(to_square);

        if self.active_player == Color::White {
            self.white_occupancies.unset_bit(from_square);
            self.white_occupancies.set_bit(to_square);
        } else {
            self.black_occupancies.unset_bit(from_square);
            self.black_occupancies.set_bit(to_square);
        }

        //Captures
        if capturing {
            let start;
            let end;
            if self.active_player == Color::White {
                start = Piece::BlackPawn as usize;
                end = Piece::BlackKing as usize;
                self.black_occupancies.unset_bit(to_square);
            }
            else {
                start = Piece::WhitePawn as usize;
                end = Piece::WhiteKing as usize;
                self.white_occupancies.unset_bit(to_square);
            }

            for bb in start..end {
                if self.bitboards[bb].get_bit(to_square) {
                    self.bitboards[bb].unset_bit(to_square);
                    break;
                }
            }

            //Reset half moves
            self.half_moves = 0;
        }

        //Promotions
        if promotion != Piece::None as u8 {
            //Spawn promoted
            self.bitboards[promotion as usize].set_bit(to_square);

            //Remove pawn
            self.bitboards[piece as usize].unset_bit(to_square);
        }

        //Enpassant capture
        else if enpassant {
            if self.active_player == Color::White {
                self.bitboards[Piece::BlackPawn as usize].unset_bit(to_square + 8);
                self.black_occupancies.unset_bit(to_square + 8);
                self.all_occupancies.unset_bit(to_square + 8);
            }
            else {
                self.bitboards[Piece::WhitePawn as usize].unset_bit(to_square - 8);
                self.white_occupancies.unset_bit(to_square - 8);
                self.all_occupancies.unset_bit(to_square - 8);
            }
        }

        //Castling
        else if castling {
            match to_square {
                62 => { //White kingside
                    self.bitboards[Piece::WhiteRook as usize].set_bit_sq(Square::f1);
                    self.bitboards[Piece::WhiteRook as usize].unset_bit_sq(Square::h1);
                    self.white_occupancies.set_bit_sq(Square::f1);
                    self.white_occupancies.unset_bit_sq(Square::h1);
                    self.all_occupancies.set_bit_sq(Square::f1);
                    self.all_occupancies.unset_bit_sq(Square::h1);
                }
                58 => { //White queenside
                    self.bitboards[Piece::WhiteRook as usize].set_bit_sq(Square::d1);
                    self.bitboards[Piece::WhiteRook as usize].unset_bit_sq(Square::a1);
                    self.white_occupancies.set_bit_sq(Square::d1);
                    self.white_occupancies.unset_bit_sq(Square::a1);
                    self.all_occupancies.set_bit_sq(Square::d1);
                    self.all_occupancies.unset_bit_sq(Square::a1);
                }
                6 => { //Black kingside
                    self.bitboards[Piece::BlackRook as usize].set_bit_sq(Square::f8);
                    self.bitboards[Piece::BlackRook as usize].unset_bit_sq(Square::h8);
                    self.black_occupancies.set_bit_sq(Square::f8);
                    self.black_occupancies.unset_bit_sq(Square::h8);
                    self.all_occupancies.set_bit_sq(Square::f8);
                    self.all_occupancies.unset_bit_sq(Square::h8);
                }
                2 => { //Black queenside
                    self.bitboards[Piece::BlackRook as usize].set_bit_sq(Square::d8);
                    self.bitboards[Piece::BlackRook as usize].unset_bit_sq(Square::a8);
                    self.black_occupancies.set_bit_sq(Square::d8);
                    self.black_occupancies.unset_bit_sq(Square::a8);
                    self.all_occupancies.set_bit_sq(Square::d8);
                    self.all_occupancies.unset_bit_sq(Square::a8);
                }
                _ => unreachable!()
            }
        }

        //Double push
        if double_push {
            if self.active_player == Color::White {
                self.enpassant_square = SQUARES[(to_square + 8) as usize];
            }
            else {
                self.enpassant_square = SQUARES[(to_square - 8) as usize];
            }
        }
        else {
            self.enpassant_square = Square::None
        }

        //Update castling abililties
        self.castling_ability &= CASTLING_RIGHTS[to_square as usize] & CASTLING_RIGHTS[from_square as usize];

        //increment fullmoves & switch player
        if self.active_player == Color::Black {
            self.full_moves += 1;
        }
        self.active_player = opposite_color(self.active_player)
    }

    fn add_move_if_legal(&mut self, move_list: &mut MoveList, cmove: Move) {
        let from_sq = cmove.from_square();
        let to_sq = cmove.to_square();
        let capture = cmove.is_capture();
        let piece_ind = cmove.piece() as usize;

        //Peek make
        //Update all_occupancies
        self.all_occupancies.unset_bit(from_sq);
        self.bitboards[piece_ind].unset_bit(from_sq);
        self.bitboards[piece_ind].set_bit(to_sq);
        self.all_occupancies.set_bit(to_sq);
        let mut taken = 0;
        //Unset captured
        if capture {
            let start;
            let end;
            if self.active_player == Color::White {
                start = Piece::BlackPawn as usize;
                end = Piece::BlackKing as usize;
            }
            else {
                start = Piece::WhitePawn as usize;
                end = Piece::WhiteKing as usize;
            }

            for bb in start..end {
                if self.bitboards[bb].get_bit(to_sq) {
                    self.bitboards[bb].unset_bit(to_sq);
                    taken = bb;
                    break;
                }
            }
        }
        else if cmove.is_enpassant() {
            if self.active_player == Color::White {
                self.bitboards[Piece::BlackPawn as usize].unset_bit(to_sq + 8);
                self.all_occupancies.unset_bit(to_sq + 8);
            }
            else {
                self.bitboards[Piece::WhitePawn as usize].unset_bit(to_sq - 8);
                self.all_occupancies.unset_bit(to_sq - 8);
            }
        }

        //Add if not in check
        if !self.is_in_check(self.active_player) {
            move_list.add_move(cmove)
        }

        //Peek unmake
        //Reset occupancies
        self.all_occupancies.set_bit(from_sq);
        self.bitboards[piece_ind].set_bit(from_sq);
        self.bitboards[piece_ind].unset_bit(to_sq);
        //Unset captured
        if capture {
            self.bitboards[taken].set_bit(to_sq);
        }
        else if cmove.is_enpassant() {
            if self.active_player == Color::White {
                self.bitboards[Piece::BlackPawn as usize].set_bit(to_sq + 8);
                self.all_occupancies.set_bit(to_sq + 8);
            }
            else {
                self.bitboards[Piece::WhitePawn as usize].set_bit(to_sq - 8);
                self.all_occupancies.set_bit(to_sq - 8);
            }
            self.all_occupancies.unset_bit(to_sq);
        }
        else {
            self.all_occupancies.unset_bit(to_sq);
        }
    }

    pub fn perft(&mut self, depth: u8, print: bool) -> u128 {
        let moves = self.generate_moves();

        if depth == 1 {
            return moves.len() as u128;
        }

        moves.values().par_iter().map(|m| {
            let mut copy = self.clone();

            copy.make_move(m);
            let r = copy.perft(depth - 1, false);

            if print {
                println!("{}{}: {}", SQUARE_STRINGS[m.from_square() as usize], SQUARE_STRINGS[m.to_square() as usize], r)
            }

            r
        }).sum()
    }

    pub fn parse_move(&mut self, input: String) -> Option<Move> {
        let moves = self.generate_moves().values();
        let m = moves.iter().find(|m| m.to_uci() == input);
        match m {
            Some(a) => Some(a.clone()),
            None => None,
        }
    }

    pub fn search_random(&mut self) -> Move {
        let moves = self.generate_moves();
        let rand = rand::thread_rng().gen_range(0..moves.len());
        moves.get(rand).clone()
    }

    pub fn evaluate(&self) -> i32 {
        let mut score: i32 = 0;

        for bb in 0..12 {
            let mut board = self.bitboards[bb];
            while !board.is_empty() {
                let square = board.extract_bit();
                score += MATERIAL_WEIGHTS[bb];

                match bb {
                    0  => score += PAWN_SCORES[square as usize],
                    1  => score += ROOK_SCORES[square as usize],
                    2  => score += KNIGHT_SCORES[square as usize],
                    3  => score += BISHOP_SCORES[square as usize],
                    4  =>  { } //No queen values,
                    5  => score += KING_SCORES[MIRRORED[square as usize]],

                    6  => score -= PAWN_SCORES[MIRRORED[square as usize]],
                    7  => score -= ROOK_SCORES[MIRRORED[square as usize]],
                    8  => score -= KNIGHT_SCORES[MIRRORED[square as usize]],
                    9  => score -= BISHOP_SCORES[MIRRORED[square as usize]],
                    10 => { } //No queen values,
                    11 => score -= KING_SCORES[MIRRORED[square as usize]],
                    _ => unreachable!()
                }
            }
        }

        if self.active_player == Color::White { score } else { -score }
    }

    ///Returns the best move and the number of nodes visited
    pub fn alphabeta_search(&mut self, depth: u8) -> SearchResult {
        let result = self.rec_alphabeta(depth, -100000, 100000, true);

        SearchResult::new(Move::new_from_u32(result.0 as u32), result.1)
    }
    
    fn rec_alphabeta(&mut self, depth: u8, alpha: i32, beta: i32, root: bool) -> (i32, u32) {
        let mut nodes = 1;

        if depth == 0 {
            return self.quiescence(alpha, beta, 0)
        }

        let moves = self.generate_moves();

        //Mate & Draw
        if moves.len() == 0 {
            if self.is_in_check(self.active_player) {
                return (-99000 - depth as i32, nodes);
            }
            else {
                return (0, 1);
            }
        }

        let mut new_alpha = alpha;
        let mut best_so_far = moves.get(0);

        for i in 0..moves.len() {
            let mut copy = self.clone();
            let m = moves.get(i);
            copy.make_move(m);

            let result = copy.rec_alphabeta(depth - 1, -beta, -new_alpha, false);
            let score = -result.0;
            nodes += result.1;
            
            //Fail hard/hard
            if score >= beta {
                return (beta, nodes);
            }

            //Found better
            if score > new_alpha {
                new_alpha = score;

                best_so_far = m;
            }
        }

        if !root {
            //Fail low
            (new_alpha, nodes)
        }
        else {
            //return move
            (best_so_far.to_u32() as i32, nodes)
        }
    }

    fn quiescence (&mut self, alpha: i32, beta: i32, ply: u8) -> (i32, u32) {
        let mut nodes = 1;
        let mut new_alpha = alpha;
        let eval = self.evaluate();

        //Fail hard/hard
        if eval >= beta {
            return (beta, nodes);
        }

        //Found better
        if eval > new_alpha {
            new_alpha = eval;
        }

        let moves = self.generate_moves();

        //Mate & Draw
        if moves.len() == 0 {
            if self.is_in_check(self.active_player) {
                return (-99000 + ply as i32, nodes);
            }
            else {
                return (0, 1);
            }
        }

        let mut captures = 0;

        for i in 0..moves.len() {
            let m = moves.get(i);
            if m.is_capture() {
                let mut copy = self.clone();
                
                copy.make_move(m);

                let result = copy.quiescence(-beta, -new_alpha, ply + 1);
                let score = -result.0;
                nodes += result.1;
                captures += 0;
                
                //Fail hard/hard
                if score >= beta {
                    return (beta, nodes);
                }

                //Found better
                if score > new_alpha {
                    new_alpha = score;
                }
            }
        }

        if captures == 0 {
            return (eval, nodes);
        }

        (new_alpha, nodes)
    }
}

#[cfg(test)]
mod search_tests {
    use super::*;

    #[test]
    pub fn lolololol() {
        let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
        let res = game.alphabeta_search(5);

        println!("{}", res.nodes_visited)
    }
}

#[cfg(test)]
mod move_gen_tests {
    use super::*;

    //Legal moves
    #[test]
    pub fn white_pawn_can_move_one_tile_forward() {
        let mut game = Game::new_from_start_pos();
        let moves = game.generate_moves();
        assert!(moves.contains(&Move::new_friendly(Square::a2, Square::a3, Piece::WhitePawn, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn black_pawn_can_move_one_tile_forward() {
        let mut game =  Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        let moves = game.generate_moves();
        assert!(moves.contains(&Move::new_friendly(Square::c7, Square::c6, Piece::BlackPawn, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn white_pawn_can_correctly_double_push() {
        let mut game = Game::new_from_start_pos();
        let moves = game.generate_moves();
        assert!(moves.contains(&Move::new_friendly(Square::a2, Square::a4, Piece::WhitePawn, Piece::None, false, true, false, false)));
    }

    #[test]
    pub fn black_pawn_can_correctly_double_push() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let moves = game.generate_moves();
        assert!(moves.contains(&Move::new_friendly(Square::c7, Square::c5, Piece::BlackPawn, Piece::None, false, true, false, false)));
    }

    #[test]
    pub fn pawn_can_capture_on_both_diagonals() {
        let mut game = Game::new_from_fen("1k6/8/8/4p1b1/5P2/8/8/1K6 w - - 0 25").unwrap();
        let moves = game.generate_moves();
        assert!(moves.contains(&Move::new_friendly(Square::f4, Square::e5, Piece::WhitePawn, Piece::None, true, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::f4, Square::g5, Piece::WhitePawn, Piece::None, true, false, false, false)));
    }

    #[test]
    pub fn white_can_enpassant_capture_correctly() {
        let mut game = Game::new_from_fen("k7/8/8/4Pp2/8/8/8/K7 w - f6 0 25").unwrap();
        let moves = game.generate_moves();
        moves.print();
        assert!(moves.contains(&Move::new_friendly(Square::e5, Square::f6, Piece::WhitePawn, Piece::None, false, false, true, false)));
    }

    #[test]
    pub fn black_can_enpassant_capture_correctly() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/pP6/8/7K b - b2 0 25").unwrap();
        let moves = game.generate_moves();
        assert!(moves.contains(&Move::new_friendly(Square::a3, Square::b2, Piece::BlackPawn, Piece::None, false, false, true, false)));
    }

    #[test]
    pub fn can_not_move_pawn_when_piece_in_the_way() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/1N6/1P6/K7 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::b2);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    pub fn white_pawn_should_have_4_promotion_options_when_reaching_back_row() {
        let mut game = Game::new_from_fen("k7/2P5/8/8/8/8/8/K7 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::c7);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn black_pawn_should_have_4_promotion_options_when_reaching_back_row() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/8/2p5/K7 b - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::c2);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn should_be_able_to_promote_on_back_row_capture() {
        let mut game = Game::new_from_fen("k2r4/2P5/8/8/8/8/8/K7 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::c7);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    pub fn white_knight_has_2_right_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = game.generate_moves().all_from(Square::b1);
        assert!(moves.len() == 2);
        assert!(moves.contains(&Move::new_friendly(Square::b1, Square::a3, Piece::WhiteKnight, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::b1, Square::c3, Piece::WhiteKnight, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn king_can_move_in_all_directions() {
        let mut game = Game::new_from_fen("8/1K6/8/4k3/8/8/8/8 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::b7);
        assert!(moves.len() == 8);
        assert!(moves.contains(&Move::new_friendly(Square::b7, Square::a8, Piece::WhiteKing, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::b7, Square::b6, Piece::WhiteKing, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn king_cannot_move_over_edge() {
        let mut game = Game::new_from_fen("8/K7/8/4k3/8/8/8/8 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::a7);
        assert!(moves.len() == 5);
        assert!(moves.contains(&Move::new_friendly(Square::a7, Square::a8, Piece::WhiteKing, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::a7, Square::b6, Piece::WhiteKing, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn rook_has_no_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = game.generate_moves().all_from(Square::a1);
        assert!(moves.len() == 0);
    }

    #[test]
    pub fn  queen_has_no_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = game.generate_moves().all_from(Square::d1);
        assert!(moves.len() == 0);
    }

    #[test]
    pub fn queen_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3Q4/8/8/7k w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::d4);
        assert!(moves.len() == 27);
        assert!(moves.contains(&Move::new_friendly(Square::d4, Square::d1, Piece::WhiteQueen, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn rook_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3R4/8/8/7k w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::d4);
        assert!(moves.len() == 14);
    }

    #[test]
    pub fn bishop_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3B4/8/8/7k w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::d4);
        assert!(moves.len() == 13);
    }

    #[test]
    pub fn queen_has_correct_number_of_moves_when_friendlies_in_the_way() {
        let mut game = Game::new_from_fen("8/1KR5/1QN5/1BB5/8/8/8/7k w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::b6);
        assert!(moves.len() == 3);
    }

    #[test]
    pub fn queen_has_correct_number_of_moves_when_enemies_in_the_way() {
        let mut game = Game::new_from_fen("7K/1rr5/1Qb5/1nb5/8/8/8/7k w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::b6);
        assert!(moves.len() == 8);
    }

    #[test]
    pub fn castling_moves_are_found_for_white() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::e1);
        assert!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)));
        assert!(moves.contains(&Move::new_friendly(Square::e1, Square::c1, Piece::WhiteKing, Piece::None, false, false, false, true)));
    }

    #[test]
    pub fn castling_moves_are_found_for_black() {
        let mut game = Game::new_from_fen("r3k2r/8/8/8/8/8/8/4K3 b kq - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::e8);
        assert!(moves.contains(&Move::new_friendly(Square::e8, Square::g8, Piece::BlackKing, Piece::None, false, false, false, true)));
        assert!(moves.contains(&Move::new_friendly(Square::e8, Square::c8, Piece::BlackKing, Piece::None, false, false, false, true)));
    }

    #[test]
    pub fn castling_moves_are_not_found_when_unavailable() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::e1);
        assert!(moves.len() == 5);
    }

    #[test]
    pub fn cant_castle_if_pieces_in_the_way() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/RR2K1NR w KQ - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::e1);
        assert_eq!(moves.len(), 5);
    }

    #[test]
    pub fn cant_move_king_into_rook_line_of_attack() {
        let mut game = Game::new_from_fen("kr6/8/8/8/8/8/8/K7 w - - 0 25").unwrap();
        game.generate_moves().print();
        let moves = game.generate_moves().all_from(Square::a1);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    pub fn bishop_has_correct_number_of_legal_moves() {
        let mut game = Game::new_from_fen("K6k/B7/8/8/8/8/8/8 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::a7);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    pub fn cant_move_blocking_piece_if_king_is_pinned() {
        let mut game = Game::new_from_fen("K6k/B7/r7/8/8/8/8/8 w - - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::a7);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    pub fn cant_castle_if_in_check() {
        let mut game = Game::new_from_fen("k7/8/8/4r3/8/8/8/4K2R w K - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::e1);
        assert_eq!(moves.into_iter().any(|m| m.is_castling()), false);
    }

    #[test]
    pub fn is_in_check_is_true_when_in_check_by_rook() {
        let game = Game::new_from_fen("k7/8/8/8/4r3/8/8/4K3 w K - 0 25").unwrap();
        assert_eq!(game.is_in_check(Color::White), true);
    }

    #[test]
    pub fn rooks_should_have_5_moves_here() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w K - 0 25").unwrap();
        assert_eq!(game.generate_moves().all_from(Square::h1).len(), 6);
        assert_eq!(game.generate_moves().all_from(Square::a1).len(), 6);
    }

    #[test]
    pub fn rook_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w K - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::a1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a1, Square::a7, Piece::WhiteRook, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn knight_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/8/8/1p6/8/N1BQKBNR w K - 0 25").unwrap();
        assert_eq!(game.generate_moves().all_from(Square::a1).contains(&Move::new_friendly(Square::a1, Square::b3, Piece::WhiteKnight, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn bishop_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w K - 0 25").unwrap();
        assert_eq!(game.generate_moves().all_from(Square::f1).contains(&Move::new_friendly(Square::f1, Square::b5, Piece::WhiteBishop, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn rook_captured_by_pawn_generates_right_move() {
        let mut game = Game::new_from_fen("1nbqkbnr/1ppppppp/8/8/r7/1P6/P1PPPPPP/RNBQKBNR w K - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::b3);
        assert_eq!(moves.contains(&Move::new_friendly(Square::b3, Square::a4, Piece::WhitePawn, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn pawns_cant_capture_straight() {
        let mut game = Game::new_from_fen("k7/8/8/p7/P7/8/8/K7 w K - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::d8);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a4, Square::a5, Piece::WhitePawn, Piece::None, true, false, false, false)), false);
    }

    #[test]
    pub fn pawns_cant_move_straight_into_piece() {
        let mut game = Game::new_from_fen("k7/8/8/p7/P7/8/8/K7 w K - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::a4);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a4, Square::a5, Piece::WhitePawn, Piece::None, false, false, false, false)), false);
    }

    #[test]
    pub fn rook_should_not_have_illegal_moves() {
        let mut game = Game::new_from_fen("r1bqkbnr/pppppppp/2n5/1P6/8/8/2PPPPPP/RNBQKBNR b KQkq - 0 25").unwrap();
        let moves = game.generate_moves().all_from(Square::a8);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    pub fn cant_castle_if_path_is_under_attack() {
        let mut game = Game::new_from_fen("rnbqkbn1/ppppppp1/8/8/8/4BNP1/PPPPPrP1/RNBQK2R w KQq - 0 8").unwrap();
        let moves = game.generate_moves().all_from(Square::e1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
    }

    #[test]
    pub fn can_castle_when_its_open() {
        let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
        let moves = game.generate_moves().all_from(Square::e1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)), true);
    }

    #[test]
    pub fn cant_castle_when_a_paw_is_in_front_of_king() {
        let mut game = Game::new_from_fen("r3k2r/4P3/8/8/8/8/8/4K3 b kq - 0 10").unwrap();
        let moves = game.generate_moves().all_from(Square::e8);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e8, Square::g8, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e8, Square::c8, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
    }
}

#[cfg(test)]
mod make_tests {
    use super::*;

    #[test]
    pub fn board_correct_after_move_with_rook() {
        let mut game = Game::new_from_fen("k7/1R6/8/8/8/8/8/K7 w - - 0 25").unwrap();
        game.make_move(&Move::new_friendly(Square::b7, Square::e7, Piece::WhiteRook, Piece::None, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::b7), false);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::e7), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::b7), false);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::e7), true);
    }

    #[test]
    pub fn board_correct_after_enpassant_capture() {
        let mut game = Game::new_from_fen("k7/8/8/4Pp2/8/8/8/K7 w - f6 0 25").unwrap();
        
        game.make_move(&Move::new_friendly(Square::e5, Square::f6, Piece::WhitePawn, Piece::None, false, false, true, false));

        //moved to right square
        assert_eq!(game.get_piece_bitboard(Piece::WhitePawn).get_bit_sq(Square::f6), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::f6), true);

        //Captured succesfully
        assert_eq!(game.black_occupancies.get_bit_sq(Square::f5), false);
        assert_eq!(game.get_piece_bitboard(Piece::BlackPawn).get_bit_sq(Square::f5), false);
    }

    #[test]
    pub fn switches_active_player_after_move() {
        let mut game = Game::new_from_start_pos();

        let moves = game.generate_moves().all_from(Square::c2);
        game.make_move(&moves[0]);
        assert_eq!(game.active_player, Color::Black);

        let moves = game.generate_moves().all_from(Square::f7);
        game.make_move(&moves[0]);
        assert_eq!(game.active_player, Color::White);
    }

    #[test]
    pub fn increments_full_moves_correctly() {
        let mut game = Game::new_from_start_pos();
        assert_eq!(game.full_moves, 1);

        let moves = game.generate_moves().all_from(Square::c2);
        game.make_move(&moves[0]);
        assert_eq!(game.full_moves, 1);

        let moves = game.generate_moves().all_from(Square::f7);
        game.make_move(&moves[0]);
        assert_eq!(game.full_moves, 2);
    }

    #[test]
    pub fn resets_half_moves_on_pawn_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 8 1").unwrap();
        game.make_move(&Move::new_friendly(Square::d2, Square::d3, Piece::WhitePawn, Piece::None, false, false, false, false));
        assert!(game.half_moves == 0);
    }

    #[test]
    pub fn resets_half_moves_on_capture() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 8 1").unwrap();
        game.make_move(&Move::new_friendly(Square::f1, Square::b5, Piece::WhiteBishop, Piece::None, true, false, false, false));
        assert!(game.half_moves == 0);
    }

    #[test]
    pub fn increments_half_moves_on_quiet_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 8 1").unwrap();
        game.make_move(&Move::new_friendly(Square::f1, Square::c4, Piece::WhiteBishop, Piece::None, false, false, false, false));
        assert!(game.half_moves == 9);
    }

    #[test]
    pub fn pawn_double_push_sets_enpassant_square() {
        let mut game = Game::new_from_start_pos();
        game.make_move(&Move::new_friendly(Square::d2, Square::d4, Piece::WhitePawn, Piece::None, false, true, false, false));
        assert_eq!(game.enpassant_square as u8, Square::d3 as u8);
    }

    #[test]
    pub fn enpassant_capture_kills_the_other_pawn() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 2").unwrap();
        game.make_move(&Move::new_friendly(Square::a5, Square::b6, Piece::WhitePawn, Piece::None, false, false, true, false));
        assert_eq!(game.get_piece_bitboard(Piece::BlackPawn).get_bit_sq(Square::b5), false);
        assert_eq!(game.black_occupancies.get_bit_sq(Square::b5), false);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::b5), false);
    }

    #[test]
    pub fn rooks_correctly_kingside() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 7").unwrap();
        game.make_move(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::e1), false);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteKing).get_bit_sq(Square::h1), false);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::f1), true);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteKing).get_bit_sq(Square::g1), true);

        assert_eq!(game.white_occupancies.get_bit_sq(Square::e1), false);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::h1), false);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::f1), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::g1), true);

        assert_eq!(game.all_occupancies.get_bit_sq(Square::e1), false);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::h1), false);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::f1), true);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::g1), true);
    }

    #[test]
    pub fn rooks_correctly_queenside() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 7").unwrap();
        game.make_move(&Move::new_friendly(Square::e1, Square::c1, Piece::WhiteKing, Piece::None, false, false, false, true));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::e1), false);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteKing).get_bit_sq(Square::a1), false);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::d1), true);
        assert_eq!(game.get_piece_bitboard(Piece::WhiteKing).get_bit_sq(Square::c1), true);

        assert_eq!(game.white_occupancies.get_bit_sq(Square::e1), false);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::a1), false);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::d1), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::c1), true);

        assert_eq!(game.all_occupancies.get_bit_sq(Square::e1), false);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::a1), false);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::d1), true);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::c1), true);
    }

    #[test]
    pub fn cant_castle_after_castling() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQK2R w KQkq - 3 7").unwrap();
        game.make_move(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 == 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 == 0);
    }

    #[test]
    pub fn moving_rook_disables_castling_for_that_side() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w KQkq - 0 2").unwrap();
        game.make_move(&Move::new_friendly(Square::h1, Square::h2, Piece::WhiteRook, Piece::None, false, false, false, false));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 == 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 != 0);
    }

    #[test]
    pub fn capturing_rook_disables_castling_for_that_side() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/1n6/8/R3K2R b QK - 0 2").unwrap();
        game.make_move(&Move::new_friendly(Square::b3, Square::a1, Piece::BlackKnight, Piece::None, true, false, false, false));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 != 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 == 0);
    }

    #[test]
    pub fn white_pawn_promotes_correctly_on_quietly_reaching_back_row() {
        let mut game = Game::new_from_fen("2n5/1P6/8/8/8/8/8/K6k w - - 0 2").unwrap();
        game.make_move(&Move::new_friendly(Square::b7, Square::b8, Piece::WhitePawn, Piece::WhiteQueen, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteQueen).get_bit_sq(Square::b8), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::b8), true);

    }

    #[test]
    pub fn black_pawn_promotes_correctly_on_quietly_reaching_back_row() {
        let mut game = Game::new_from_fen("K6k/8/8/8/8/8/3p4/8 b - - 0 2").unwrap();
        game.make_move(&Move::new_friendly(Square::d2, Square::d1, Piece::BlackPawn, Piece::BlackRook, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::BlackRook).get_bit_sq(Square::d1), true);
        assert_eq!(game.black_occupancies.get_bit_sq(Square::d1), true);
    }

    #[test]
    pub fn pawn_promotes_correctly_on_back_row_capture() {
        let mut game = Game::new_from_fen("2n5/1P6/8/8/8/8/8/K6k w QK - 0 2").unwrap();
        game.make_move(&Move::new_friendly(Square::b7, Square::c8, Piece::WhitePawn, Piece::WhiteBishop, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteBishop).get_bit_sq(Square::c8), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::c8), true);
    }

    #[test]
    pub fn moving_the_king_disables_castling() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 7").unwrap();
        game.make_move(&Move::new_friendly(Square::e1, Square::f1, Piece::WhiteKing, Piece::None, false, false, false, false));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 == 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 == 0);
    }

    #[test]
    pub fn capturing_bishop_results_in_correct_bitboards() {
        let mut game = Game::new_from_fen("rnbqkbnr/1ppppppp/p6B/8/8/3P4/PPP1PPPP/RN1QKBNR b KQkq - 0 1").unwrap();
        game.make_move(&Move::new_friendly(Square::g8, Square::h6, Piece::BlackPawn, Piece::None, true, false, false, false));
        
        assert_eq!(game.get_piece_bitboard(Piece::WhiteBishop).get_bit_sq(Square::h6), false);
        assert_eq!(game.get_piece_bitboard(Piece::BlackPawn).get_bit_sq(Square::h6), true);

        assert_eq!(game.white_occupancies.get_bit_sq(Square::h6), false);
        assert_eq!(game.black_occupancies.get_bit_sq(Square::h6), true);

        assert_eq!(game.all_occupancies.get_bit_sq(Square::h6), true);
    }
}