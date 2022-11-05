use super::*;

///The maximum length of the moveList. Array is initialized to this
const MOVE_LIST_SIZE: usize = 256; //Maybe 128?

#[derive(PartialEq)]
pub enum MoveTypes {
    All,
    Captures
}

pub struct MoveGenerator<'a> {
    pos: &'a mut Position,
    moves: [Move; MOVE_LIST_SIZE],
    insert_index: usize,
    extract_index: usize,
    move_types: MoveTypes,
    phase: MoveTypes
    //The current phase of generation
}

impl <'a>MoveGenerator<'a> {
    ///Initializes the generator to a position
    pub fn initialize(pos: &'a mut Position, move_types: MoveTypes) -> Self {
        Self {
            pos: pos,
            moves: [NULL_MOVE; MOVE_LIST_SIZE],
            insert_index: 0,
            extract_index: 0,
            move_types,
            phase: MoveTypes::Captures
        }
    }

    ///Generates all moves instantly, and returns them
    pub fn collect(&mut self) -> Vec<Move>{
        todo!()
    }

    ///Indicates whether the move generator still contains moves
    pub fn has_next(&mut self) -> bool {
        todo!()
    }

    ///Gets next move, best first. Dynamically generates moves in phases lazily sorted. Returns NULL_MOVE, when empty
    pub fn get_next_move(&mut self, sort: bool) -> Move {
        //We have run out
        if self.extract_index >= self.insert_index {
            match self.phase {
                MoveTypes::All => return NULL_MOVE,
                MoveTypes::Captures => { 
                    //Iterate phase or finish generation depending on what moves are wanted
                    if self.move_types == MoveTypes::All {
                        self.phase = MoveTypes::All
                    } else {
                        return NULL_MOVE
                    }
                }
            }

            //Generate more moves
            match self.phase {
                MoveTypes::All => self.generate_non_captures(),
                MoveTypes::Captures => self.generate_captures(),
            }
        }

        let best;

        //Sort if wanted
        if sort {
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

        best
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
        todo!()
    }

    ///Generate quiet moves and adds them to the list
    fn generate_non_captures(&mut self) {
        todo!()
    }

    #[inline(always)]
    ///Adds the generated move to the move list
    fn add_move(&mut self, cmove: Move) {
        self.moves[self.insert_index] = cmove;
        self.insert_index += 1;
    }

    ///Scores a capturing move
    fn score_and_add_capture_move(&mut self) {
        todo!()
    }

    ///Scores a quiet move
    fn score_and_add_quiet_move(&mut self) {
        todo!()
    }

    ///Finds the associated with a uci string representation. eg. B2B1q
    pub fn parse_move(&mut self, input: String) -> Option<Move> {
        todo!()
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