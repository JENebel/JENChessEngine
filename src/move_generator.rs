use super::*;

///The maximum length of the moveList. Array is initialized to this
const MOVE_LIST_SIZE: usize = 256; //Maybe 128?

///[attacker][victim]
pub const MVV_LVA: [[u8; 12]; 12] = [
    [15, 25, 35, 45, 55, 65,  15, 25, 35, 45, 55, 65],
    [14, 24, 34, 44, 54, 64,  14, 24, 34, 44, 54, 64],
    [13, 23, 33, 43, 53, 63,  13, 23, 33, 43, 53, 63],
    [12, 22, 32, 42, 52, 62,  12, 22, 32, 42, 52, 62],
    [11, 21, 31, 41, 51, 61,  11, 21, 31, 41, 51, 61],
    [10, 20, 30, 40, 50, 60,  10, 20, 30, 40, 50, 60],
    
    [15, 25, 35, 45, 55, 65,  15, 25, 35, 45, 55, 65],
    [14, 24, 34, 44, 54, 64,  14, 24, 34, 44, 54, 64],
    [13, 23, 33, 43, 53, 63,  13, 23, 33, 43, 53, 63],
    [12, 22, 32, 42, 52, 62,  12, 22, 32, 42, 52, 62],
    [11, 21, 31, 41, 51, 61,  11, 21, 31, 41, 51, 61],
    [10, 20, 30, 40, 50, 60,  10, 20, 30, 40, 50, 60],
];

#[derive(PartialEq)]
pub enum MoveTypes {
    All,
    Captures
}

#[derive(PartialEq)]
enum GenPhase {
    NotStarted,
    DoneCaptures,
    DoneAll
}

pub trait MoveScorer {
    fn score_capture(&self, cmove: &mut Move);
    fn score_non_capture(&self, cmove: &mut Move);
}

pub struct NonScorer { }
impl MoveScorer for NonScorer {
    fn score_capture(&self, cmove: &mut Move) {}
    fn score_non_capture(&self, cmove: &mut Move) {}
}

impl MoveScorer for Searcher {
    ///Initializes the generator to a position
    fn score_capture(&self, cmove: &mut Move) {
        let start;
            let end;
            let mut taken = 0;
            if self.pos.active_player == Color::White {
                start = Piece::BlackPawn as usize;
                end = Piece::BlackKing as usize;
            }
            else {
                start = Piece::WhitePawn as usize;
                end = Piece::WhiteKing as usize;
            }

            for bb in start..end {
                if self.pos.bitboards[bb].get_bit(cmove.to_square()) {
                    taken = bb;
                    break;
                }
            }

            cmove.set_score(MVV_LVA[cmove.piece() as usize][taken as usize]);
    }

    fn score_non_capture(&self, cmove: &mut Move) {
        if &mut self.killer_moves[0][self.ply as usize] == cmove {
            cmove.set_score(250)
        } else if &mut self.killer_moves[1][self.ply as usize] == cmove {
            cmove.set_score(249)
        }
        else {
            cmove.set_score(self.history_moves[cmove.piece() as usize][cmove.to_square() as usize])
        }
    }
}

pub struct MoveGenerator<'a> {
    pos: &'a Position,
    scorer: & 'a dyn MoveScorer,
    moves: [Move; MOVE_LIST_SIZE],
    insert_index: usize,
    extract_index: usize,
    move_types: MoveTypes,
    phase: GenPhase,
    sort: bool
}

impl <'a>MoveGenerator<'a> {
    ///Initializes the generator to a position with a searcher reference for sorting purposes
    pub fn new_sorted(pos: &'a Position, move_types: MoveTypes, scorer: &dyn MoveScorer) -> Self {
        Self {
            pos,
            scorer,
            moves: [NULL_MOVE; MOVE_LIST_SIZE],
            insert_index: 0,
            extract_index: 0,
            move_types,
            phase: GenPhase::NotStarted,
            sort: true
        }
    }

    ///Initializes the generator to a position without sorting moves
    pub fn new_unsorted(pos: &'a Position, move_types: MoveTypes) -> Self {
        Self {
            pos,
            scorer: &NonScorer{},
            moves: [NULL_MOVE; MOVE_LIST_SIZE],
            insert_index: 0,
            extract_index: 0,
            move_types,
            phase: GenPhase::NotStarted,
            sort: false
        }
    }

    ///Generates all moves unsorted instantly, and returns them
    pub fn all_moves(pos: &Position) -> Vec<Move>{
        MoveGenerator::new_unsorted(pos, MoveTypes::All).collect()
    }

    ///Finds the associated with a uci string representation. eg. B2B1q
    pub fn parse_move(pos: &Position, input: String) -> Option<Move> {
        MoveGenerator::new_unsorted(pos, MoveTypes::All).find(|m| format!("{}", m) == input)
    }

    ///Adds the pv move to the move list if one exists
    pub fn add_pv_move(&mut self, tt: &TranspositionTable) {
        let best = tt.probe_best_move(self.pos.zobrist_hash);
        if best != NULL_MOVE {
            self.add_move(best)
        }
    }

    ///Generates capturing moves and adds them to the list
    fn generate_captures(&mut self) {
        let mut from_sq: u8;
        let mut to_sq:   u8;

        let mut attacks: Bitboard;

        let mut pawn_bitboard;
        let mut rook_bitboard;
        let mut knight_bitboard;
        let mut bishop_bitboard;
        let mut queen_bitboard;
        let mut king_bitboard;

        let opponent_occupancies: Bitboard;

        let pawn;
        let rook;
        let knight;
        let bishop;
        let queen;
        let king;

        //Color specific
        //WHITE
        if self.pos.active_player == Color::White {
            opponent_occupancies = self.pos.black_occupancies;
            pawn_bitboard =     self.pos.get_piece_bitboard(Piece::WhitePawn);
            rook_bitboard =     self.pos.get_piece_bitboard(Piece::WhiteRook);
            knight_bitboard =   self.pos.get_piece_bitboard(Piece::WhiteKnight);
            bishop_bitboard =   self.pos.get_piece_bitboard(Piece::WhiteBishop);
            queen_bitboard =    self.pos.get_piece_bitboard(Piece::WhiteQueen);
            king_bitboard =     self.pos.get_piece_bitboard(Piece::WhiteKing);

            pawn =   Piece::WhitePawn as u8;
            rook =   Piece::WhiteRook as u8;
            knight = Piece::WhiteKnight as u8;
            bishop = Piece::WhiteBishop as u8;
            queen =  Piece::WhiteQueen as u8;
            king =   Piece::WhiteKing as u8;

            //Pawn captures
            while !pawn_bitboard.is_empty() {
                from_sq = pawn_bitboard.extract_bit();

                //Regular
                attacks = get_pawn_attack_table(from_sq, Color::White);

                //Enpassant
                if self.pos.enpassant_square != Square::None && !attacks.and(Bitboard::from_u64(1 << self.pos.enpassant_square as u8)).is_empty(){
                    self.score_and_add_capture_move(Move::new(from_sq, self.pos.enpassant_square as u8, pawn as u8, Piece::None as u8, true, false, true, false));
                }

                //Overlap with opponent occupancies
                attacks = attacks.and(opponent_occupancies);

                while !attacks.is_empty() {
                    to_sq = attacks.extract_bit();
                    //Regular captures
                    if to_sq >= 8 {
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::None as u8, true, false, false, false));

                    //Promotions
                    } else {
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteQueen as u8,  true, false, false, false));
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteKnight as u8, true, false, false, false));
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteRook as u8,   true, false, false, false));
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteBishop as u8, true, false, false, false))
                    }
                }
            }
        }
        //BLACK
        else {
            opponent_occupancies = self.pos.white_occupancies;
            pawn_bitboard = self.pos.get_piece_bitboard(Piece::BlackPawn);
            rook_bitboard = self.pos.get_piece_bitboard(Piece::BlackRook);
            knight_bitboard = self.pos.get_piece_bitboard(Piece::BlackKnight);
            bishop_bitboard = self.pos.get_piece_bitboard(Piece::BlackBishop);
            queen_bitboard = self.pos.get_piece_bitboard(Piece::BlackQueen);
            king_bitboard = self.pos.get_piece_bitboard(Piece::BlackKing);

            pawn =   Piece::BlackPawn as u8;
            rook =   Piece::BlackRook as u8;
            knight = Piece::BlackKnight as u8;
            bishop = Piece::BlackBishop as u8;
            queen =  Piece::BlackQueen as u8;
            king =   Piece::BlackKing as u8;

            //Pawn captures
            while !pawn_bitboard.is_empty() {
                from_sq = pawn_bitboard.extract_bit();

                //Regular
                attacks = get_pawn_attack_table(from_sq, Color::Black);

                //Enpassant
                if self.pos.enpassant_square != Square::None && !attacks.and(Bitboard::from_u64(1 << self.pos.enpassant_square as u8)).is_empty(){
                    self.score_and_add_capture_move(Move::new(from_sq, self.pos.enpassant_square as u8, pawn, Piece::None as u8, true, false, true, false));
                }

                //Overlap with opponent occupancies
                attacks = attacks.and(opponent_occupancies);

                while !attacks.is_empty() {
                    to_sq = attacks.extract_bit();
                    //Regular captures
                    if to_sq <= 55 {
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::None as u8, true, false, false, false));

                    //Promotions
                    } else {
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::BlackQueen as u8,  true, false, false, false));
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::BlackKnight as u8, true, false, false, false));
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::BlackRook as u8,   true, false, false, false));
                        self.score_and_add_capture_move(Move::new(from_sq, to_sq, pawn, Piece::BlackBishop as u8, true, false, false, false))
                    }
                }
            }
        }

        //Knight attacks
        while !knight_bitboard.is_empty() {
            from_sq = knight_bitboard.extract_bit();

            //Raw attack table
            attacks = get_knight_attack_table(from_sq);

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.score_and_add_capture_move(Move::new(from_sq, to_sq, knight, Piece::None as u8, true, false, false, false))
            }
        }

        //Bishop attacks
        while !bishop_bitboard.is_empty() {
            from_sq = bishop_bitboard.extract_bit();

            //Raw attack table
            attacks = get_bishop_attack_table(from_sq, self.pos.all_occupancies);  

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.score_and_add_capture_move(Move::new(from_sq, to_sq, bishop, Piece::None as u8, true, false, false, false))
            }
        }

        //Rook attacks
        while !rook_bitboard.is_empty() {
            from_sq = rook_bitboard.extract_bit();

            //Raw attack table
            attacks = get_rook_attack_table(from_sq, self.pos.all_occupancies);

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.score_and_add_capture_move(Move::new(from_sq, to_sq, rook, Piece::None as u8, true, false, false, false))
            }
        }

        //Queen attacks
        while !queen_bitboard.is_empty() {
            from_sq = queen_bitboard.extract_bit();

            //Raw attack table
            attacks = get_queen_attack_table(from_sq, self.pos.all_occupancies);

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.score_and_add_capture_move(Move::new(from_sq, to_sq, queen, Piece::None as u8, true, false, false, false))
            }
        }

        //King attacks
        while !king_bitboard.is_empty() {
            from_sq = king_bitboard.extract_bit();

            //Raw attack table
            attacks = get_king_attack_table(from_sq,);

            //Extract only captures and loop over them
            attacks = attacks.and(opponent_occupancies);
            while !attacks.is_empty() {
                to_sq = attacks.extract_bit();
                self.score_and_add_capture_move(Move::new(from_sq, to_sq, king, Piece::None as u8, true, false, false, false))
            }
        }

        self.phase = GenPhase::DoneCaptures;
    }

    ///Generate quiet moves and adds them to the list
    fn generate_non_captures(&mut self) {
        let mut from_sq: u8;
        let mut to_sq:   u8;

        let mut quiet: Bitboard;

        let mut pawn_bitboard;
        let mut rook_bitboard;
        let mut knight_bitboard;
        let mut bishop_bitboard;
        let mut queen_bitboard;
        let mut king_bitboard;

        let pawn;
        let rook;
        let knight;
        let bishop;
        let queen;
        let king;

        //Color specific
        //WHITE
        if self.pos.active_player == Color::White {
            pawn_bitboard =     self.pos.get_piece_bitboard(Piece::WhitePawn);
            rook_bitboard =     self.pos.get_piece_bitboard(Piece::WhiteRook);
            knight_bitboard =   self.pos.get_piece_bitboard(Piece::WhiteKnight);
            bishop_bitboard =   self.pos.get_piece_bitboard(Piece::WhiteBishop);
            queen_bitboard =    self.pos.get_piece_bitboard(Piece::WhiteQueen);
            king_bitboard =     self.pos.get_piece_bitboard(Piece::WhiteKing);

            pawn =   Piece::WhitePawn as u8;
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
                if !self.pos.all_occupancies.get_bit(to_sq) {
                    //to_sq is empty
                    if to_sq >= 8 {
                        //Quiet move
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn, Piece::None as u8, false, false, false, false));

                        //Double push
                        to_sq = (to_sq as i8 - 8) as u8;
                        if !self.pos.all_occupancies.get_bit(to_sq) && from_sq / 8 == 6 {
                            self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn, Piece::None as u8, false, true, false, false));
                        }
                    }
                    //Promotions
                    else {
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteQueen as u8,  false, false, false, false));
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteKnight as u8, false, false, false, false));
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteRook as u8,   false, false, false, false));
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn, Piece::WhiteBishop as u8, false, false, false, false))
                    }
                }
            }

            //Castling kingside
            if self.pos.castling_ability & (CastlingAbility::WhiteKingSide as u8) != 0 &&              //castling ability
                (self.pos.all_occupancies.and(Bitboard::from_u64(6917529027641081856))).is_empty() &&   //f1 and g1 are free. 6917529027641081856 is f1 and g1 set
                !self.pos.is_square_attacked(Square::e1 as u8, Color::Black) &&                         //e1 is notunder attack
                !self.pos.is_square_attacked(Square::f1 as u8, Color::Black) {                          //f1 is not under attack

                    self.score_and_add_quiet_move(Move::new(Square::e1 as u8, Square::g1 as u8, Piece::WhiteKing as u8, Piece::None as u8, false, false, false, true))
            }
            //Castling queen
            if  self.pos.castling_ability & (CastlingAbility::WhiteQueenSide as u8) != 0 &&             //castling ability
                (self.pos.all_occupancies.and(Bitboard::from_u64(1008806316530991104))).is_empty() &&   //d1, c1 and b1 are free. 1008806316530991104 is d1, c1 and b1 set
                !self.pos.is_square_attacked(Square::e1 as u8, Color::Black) &&                         //e1 is notunder attack
                !self.pos.is_square_attacked(Square::d1 as u8, Color::Black) {                          //d1 is not under attack

                    self.score_and_add_quiet_move(Move::new(Square::e1 as u8, Square::c1 as u8, Piece::WhiteKing as u8, Piece::None as u8, false, false, false, true))
            }
        }
        //BLACK
        else {
            pawn_bitboard = self.pos.get_piece_bitboard(Piece::BlackPawn);
            rook_bitboard = self.pos.get_piece_bitboard(Piece::BlackRook);
            knight_bitboard = self.pos.get_piece_bitboard(Piece::BlackKnight);
            bishop_bitboard = self.pos.get_piece_bitboard(Piece::BlackBishop);
            queen_bitboard = self.pos.get_piece_bitboard(Piece::BlackQueen);
            king_bitboard = self.pos.get_piece_bitboard(Piece::BlackKing);

            pawn =   Piece::BlackPawn as u8;
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
                if !self.pos.all_occupancies.get_bit(to_sq) {
                    //to_sq is empty
                    if to_sq <= 55 {
                        //Quiet move
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn as u8, Piece::None as u8, false, false, false, false));

                        //Double push
                        to_sq = (to_sq as i8 + 8) as u8;
                        if !self.pos.all_occupancies.get_bit(to_sq) && from_sq / 8 == 1 {
                            self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn as u8, Piece::None as u8, false, true, false, false));
                        }
                    }
                    //Promotions
                    else {
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn as u8, Piece::BlackQueen as u8,  false, false, false, false));
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn as u8, Piece::BlackKnight as u8, false, false, false, false));
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn as u8, Piece::BlackRook as u8,   false, false, false, false));
                        self.score_and_add_quiet_move(Move::new(from_sq, to_sq, pawn as u8, Piece::BlackBishop as u8, false, false, false, false))
                    }
                }
            }

            //Castling kingside
            if  self.pos.castling_ability & (CastlingAbility::BlackKingSide as u8) != 0 &&              //castling ability
                (self.pos.all_occupancies.and(Bitboard::from_u64(96))).is_empty() &&                    //f8 and g8 are free. 96 is f8 and g8 set
                !self.pos.is_square_attacked(Square::e8 as u8, Color::White) &&                         //e8 is notunder attack
                !self.pos.is_square_attacked(Square::f8 as u8, Color::White) {                          //f8 is not under attack

                    self.score_and_add_quiet_move(Move::new(Square::e8 as u8, Square::g8 as u8, Piece::BlackKing as u8, Piece::None as u8, false, false, false, true))
            }
            //Castling queen
            if  self.pos.castling_ability & (CastlingAbility::BlackQueenSide as u8) != 0 &&             //castling ability
                (self.pos.all_occupancies.and(Bitboard::from_u64(14))).is_empty() &&                    //d8, c8 and b8 are free. 14 is d8, c8 and b8 set
                !self.pos.is_square_attacked(Square::e8 as u8, Color::White) &&                         //e8 is notunder attack
                !self.pos.is_square_attacked(Square::d8 as u8, Color::White) {                          //d8 is not under attack

                    self.score_and_add_quiet_move(Move::new(Square::e8 as u8, Square::c8 as u8, Piece::BlackKing as u8, Piece::None as u8, false, false, false, true))
            }
        }

        let not_occ = self.pos.all_occupancies.not();

        //Knight attacks
        while !knight_bitboard.is_empty() {
            from_sq = knight_bitboard.extract_bit();

            //Quiet moves
            quiet = get_knight_attack_table(from_sq).and(not_occ);

            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.score_and_add_quiet_move(Move::new(from_sq, to_sq, knight, Piece::None as u8, false, false, false, false))
            }
        }

        //Bishop attacks
        while !bishop_bitboard.is_empty() {
            from_sq = bishop_bitboard.extract_bit();

            //Quiet moves
            quiet = get_bishop_attack_table(from_sq, self.pos.all_occupancies).and(not_occ);

            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.score_and_add_quiet_move(Move::new(from_sq, to_sq, bishop, Piece::None as u8, false, false, false, false))
            }
        }

        //Rook attacks
        while !rook_bitboard.is_empty() {
            from_sq = rook_bitboard.extract_bit();

            //Quiet moves
            quiet = get_rook_attack_table(from_sq, self.pos.all_occupancies).and(not_occ);

            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.score_and_add_quiet_move(Move::new(from_sq, to_sq, rook, Piece::None as u8, false, false, false, false))
            }
        }

        //Queen attacks
        while !queen_bitboard.is_empty() {
            from_sq = queen_bitboard.extract_bit();

            //Quiet moves
            quiet = get_queen_attack_table(from_sq, self.pos.all_occupancies).and(not_occ);

            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.score_and_add_quiet_move(Move::new(from_sq, to_sq, queen, Piece::None as u8, false, false, false, false))
            }
        }

        //King attacks
        while !king_bitboard.is_empty() {
            from_sq = king_bitboard.extract_bit();

            //Quiet moves
            quiet = get_king_attack_table(from_sq).and(not_occ);

            while !quiet.is_empty() {
                to_sq = quiet.extract_bit();
                self.score_and_add_quiet_move(Move::new(from_sq, to_sq, king, Piece::None as u8, false, false, false, false))
            }
        }

        self.phase = GenPhase::DoneAll;
    }

    #[inline(always)]
    ///Adds the generated move to the move list
    fn add_move(&mut self, cmove: Move) {
        self.moves[self.insert_index] = cmove;
        self.insert_index += 1;
    }

    ///Scores a capturing move
    fn score_and_add_capture_move(&mut self, mut cmove: Move) {
        self.scorer.score_capture(&mut cmove);
        self.add_move(cmove)
    }

    ///Scores a quiet move
    fn score_and_add_quiet_move(&mut self, mut cmove: Move) {
        self.scorer.score_non_capture(&mut cmove);
        self.add_move(cmove)
    }
}

impl Iterator for MoveGenerator<'_> {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        //We have run out
        if self.extract_index == self.insert_index {
            //Generate more moves
            match self.phase {
                GenPhase::NotStarted => { self.generate_captures() },
                GenPhase::DoneCaptures => if self.move_types == MoveTypes::All { self.generate_non_captures() },
                GenPhase::DoneAll => return None
            }
        }

        let best;

        //Sort if wanted
        if self.sort {
            //Find move
            let mut best_index = self.extract_index;
            let mut best_score = self.moves[best_index].score;
            
            for i in (self.extract_index + 1)..self.insert_index {
                if self.moves[i].score > best_score {
                    best_score = self.moves[i].score;
                    best_index = i;
                }
            }

            //Swap
            best = self.moves[best_index];
            self.moves[self.extract_index] = self.moves[best_index];
        }
        else {
            best = self.moves[self.extract_index]
        }
        
        self.extract_index += 1;

        Some(best)
    }
}

/*
#[cfg(test)]
mod move_gen_tests {
    use super::*;

    #[test]
    pub fn perft_test () {
        let mut game = Game::new_from_start_pos();
        let mut moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::a2 as u8 && m.to_square() == Square::a3 as u8 ).unwrap());
        moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::d7 as u8 && m.to_square() == Square::d6 as u8 ).unwrap());
        moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::b2 as u8 && m.to_square() == Square::b3 as u8 ).unwrap());
        moves = generate_moves(&mut game, MoveTypes::All).legal_values(&mut game);
        make_move(&mut game, moves.iter().find(|m| m.from_square() == Square::c8 as u8 && m.to_square() == Square::h3 as u8 ).unwrap());
        generate_moves(&mut game, MoveTypes::All).print();
        let pe = perft(&mut game, 1, true);
        game.all_occupancies.print();
        println!("Found total: {}", pe)

    }

    //Legal moves
    #[test]
    pub fn white_pawn_can_move_one_tile_forward() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::a2, Square::a3, Piece::WhitePawn, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn black_pawn_can_move_one_tile_forward() {
        let mut game =  Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::c7, Square::c6, Piece::BlackPawn, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn white_pawn_can_correctly_double_push() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::a2, Square::a4, Piece::WhitePawn, Piece::None, false, true, false, false)));
    }

    #[test]
    pub fn black_pawn_can_correctly_double_push() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 1").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::c7, Square::c5, Piece::BlackPawn, Piece::None, false, true, false, false)));
    }

    #[test]
    pub fn pawn_can_capture_on_both_diagonals() {
        let mut game = Game::new_from_fen("1k6/8/8/4p1b1/5P2/8/8/1K6 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::f4, Square::e5, Piece::WhitePawn, Piece::None, true, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::f4, Square::g5, Piece::WhitePawn, Piece::None, true, false, false, false)));
    }

    #[test]
    pub fn white_can_enpassant_capture_correctly() {
        let mut game = Game::new_from_fen("k7/8/8/4Pp2/8/8/8/K7 w - f6 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        moves.print();
        assert!(moves.contains(&Move::new_friendly(Square::e5, Square::f6, Piece::WhitePawn, Piece::None, true, false, true, false)));
    }

    #[test]
    pub fn black_can_enpassant_capture_correctly() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/pP6/8/7K b - b2 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All);
        assert!(moves.contains(&Move::new_friendly(Square::a3, Square::b2, Piece::BlackPawn, Piece::None, true, false, true, false)));
    }

    #[test]
    pub fn can_not_move_pawn_when_piece_in_the_way() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/1N6/1P6/K7 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b2);
        assert_eq!(moves.len(), 0);
    }

    #[test]
    pub fn white_pawn_should_have_4_promotion_options_when_reaching_back_row() {
        let mut game = Game::new_from_fen("k7/2P5/8/8/8/8/8/K7 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c7);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn black_pawn_should_have_4_promotion_options_when_reaching_back_row() {
        let mut game = Game::new_from_fen("k7/8/8/8/8/8/2p5/K7 b - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c2);
        assert_eq!(moves.len(), 4);
    }

    #[test]
    pub fn should_be_able_to_promote_on_back_row_capture() {
        let mut game = Game::new_from_fen("k2r4/2P5/8/8/8/8/8/K7 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c7);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    pub fn white_knight_has_2_right_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b1);
        assert!(moves.len() == 2);
        assert!(moves.contains(&Move::new_friendly(Square::b1, Square::a3, Piece::WhiteKnight, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::b1, Square::c3, Piece::WhiteKnight, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn king_can_move_in_all_directions() {
        let mut game = Game::new_from_fen("8/1K6/8/4k3/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b7);
        assert!(moves.len() == 8);
        assert!(moves.contains(&Move::new_friendly(Square::b7, Square::a8, Piece::WhiteKing, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::b7, Square::b6, Piece::WhiteKing, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn king_cannot_move_over_edge() {
        let mut game = Game::new_from_fen("8/K7/8/4k3/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a7);
        assert!(moves.len() == 5);
        assert!(moves.contains(&Move::new_friendly(Square::a7, Square::a8, Piece::WhiteKing, Piece::None, false, false, false, false)));
        assert!(moves.contains(&Move::new_friendly(Square::a7, Square::b6, Piece::WhiteKing, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn rook_has_no_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a1);
        assert!(moves.len() == 0);
    }

    #[test]
    pub fn  queen_has_no_legal_moves_at_start() {
        let mut game = Game::new_from_start_pos();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d1);
        assert!(moves.len() == 0);
    }

    #[test]
    pub fn queen_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3Q4/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d4);
        assert!(moves.len() == 27);
        assert!(moves.contains(&Move::new_friendly(Square::d4, Square::d1, Piece::WhiteQueen, Piece::None, false, false, false, false)));
    }

    #[test]
    pub fn rook_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3R4/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d4);
        assert!(moves.len() == 14);
    }

    #[test]
    pub fn bishop_has_correct_number_of_legal_moves_on_open_board() {
        let mut game = Game::new_from_fen("K7/8/8/8/3B4/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d4);
        assert!(moves.len() == 13);
    }

    #[test]
    pub fn queen_has_correct_number_of_moves_when_friendlies_in_the_way() {
        let mut game = Game::new_from_fen("8/1KR5/1QN5/1BB5/8/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b6);
        assert!(moves.len() == 3);
    }

    #[test]
    pub fn queen_has_correct_number_of_moves_when_enemies_in_the_way() {
        let mut game = Game::new_from_fen("7K/1rr5/1Qb5/1nb5/8/8/8/7k w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b6);
        assert!(moves.len() == 8);
    }

    #[test]
    pub fn castling_moves_are_found_for_white() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w KQ - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)));
        assert!(moves.contains(&Move::new_friendly(Square::e1, Square::c1, Piece::WhiteKing, Piece::None, false, false, false, true)));
    }

    #[test]
    pub fn castling_moves_are_found_for_black() {
        let mut game = Game::new_from_fen("r3k2r/8/8/8/8/8/8/4K3 b kq - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e8);
        assert!(moves.contains(&Move::new_friendly(Square::e8, Square::g8, Piece::BlackKing, Piece::None, false, false, false, true)));
        assert!(moves.contains(&Move::new_friendly(Square::e8, Square::c8, Piece::BlackKing, Piece::None, false, false, false, true)));
    }

    #[test]
    pub fn castling_moves_are_not_found_when_unavailable() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert!(moves.len() == 5);
    }

    #[test]
    pub fn cant_castle_if_pieces_in_the_way() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/RR2K1NR w KQ - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.len(), 5);
    }

    #[test]
    pub fn cant_move_king_into_rook_line_of_attack() {
        let mut game = Game::new_from_fen("kr6/8/8/8/8/8/8/K7 w - - 0 25").unwrap();
        generate_moves(&mut game, MoveTypes::All).print();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a1);
        assert_eq!(moves.iter().filter(|m| is_legal(&mut game, m)).count(), 1);
    }

    #[test]
    pub fn bishop_has_correct_number_of_legal_moves() {
        let mut game = Game::new_from_fen("K6k/B7/8/8/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a7);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    pub fn cant_move_blocking_piece_if_king_is_pinned() {
        let mut game = Game::new_from_fen("K6k/B7/r7/8/8/8/8/8 w - - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a7);
        assert_eq!(moves.iter().any(|m| is_legal(&mut game, m)), false);
    }

    #[test]
    pub fn cant_castle_if_in_check() {
        let mut game = Game::new_from_fen("k7/8/8/4r3/8/8/8/4K2R w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
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
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::h1).len(), 6);
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::a1).len(), 6);
    }

    #[test]
    pub fn rook_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/8/RNBQKBNR w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a1, Square::a7, Piece::WhiteRook, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn knight_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/8/8/1p6/8/N1BQKBNR w K - 0 25").unwrap();
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::a1).contains(&Move::new_friendly(Square::a1, Square::b3, Piece::WhiteKnight, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn bishop_should_have_a_capture_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w K - 0 25").unwrap();
        assert_eq!(generate_moves(&mut game, MoveTypes::All).all_from(Square::f1).contains(&Move::new_friendly(Square::f1, Square::b5, Piece::WhiteBishop, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn rook_captured_by_pawn_generates_right_move() {
        let mut game = Game::new_from_fen("1nbqkbnr/1ppppppp/8/8/r7/1P6/P1PPPPPP/RNBQKBNR w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::b3);
        assert_eq!(moves.contains(&Move::new_friendly(Square::b3, Square::a4, Piece::WhitePawn, Piece::None, true, false, false, false)), true);
    }

    #[test]
    pub fn pawns_cant_capture_straight() {
        let mut game = Game::new_from_fen("k7/8/8/p7/P7/8/8/K7 w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::d8);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a4, Square::a5, Piece::WhitePawn, Piece::None, true, false, false, false)), false);
    }

    #[test]
    pub fn pawns_cant_move_straight_into_piece() {
        let mut game = Game::new_from_fen("k7/8/8/p7/P7/8/8/K7 w K - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a4);
        assert_eq!(moves.contains(&Move::new_friendly(Square::a4, Square::a5, Piece::WhitePawn, Piece::None, false, false, false, false)), false);
    }

    #[test]
    pub fn rook_should_not_have_illegal_moves() {
        let mut game = Game::new_from_fen("r1bqkbnr/pppppppp/2n5/1P6/8/8/2PPPPPP/RNBQKBNR b KQkq - 0 25").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::a8);
        assert_eq!(moves.len(), 1);
    }

    #[test]
    pub fn cant_castle_if_path_is_under_attack() {
        let mut game = Game::new_from_fen("rnbqkbn1/ppppppp1/8/8/8/4BNP1/PPPPPrP1/RNBQK2R w KQq - 0 8").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
    }

    #[test]
    pub fn can_castle_when_its_open() {
        let mut game = Game::new_from_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 10").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e1);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true)), true);
    }

    #[test]
    pub fn cant_castle_when_a_paw_is_in_front_of_king() {
        let mut game = Game::new_from_fen("r3k2r/4P3/8/8/8/8/8/4K3 b kq - 0 10").unwrap();
        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::e8);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e8, Square::g8, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
        assert_eq!(moves.contains(&Move::new_friendly(Square::e8, Square::c8, Piece::WhiteKing, Piece::None, false, false, false, true)), false);
    }
}
*/