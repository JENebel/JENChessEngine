use super::*;

use super::*;

pub const PIECE_KEYS: [[u64; 64]; 12] = generate_piece_keys();
pub const ENPASSANT_KEYS: [u64; 64] = generate_enpassant_keys();
pub const CASTLE_KEYS: [u64; 16] = generate_castle_keys();
pub const SIDE_KEY: u64 = get_random_u64_number(4084590338).0;

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

impl Position {
    pub fn make_move(&mut self, cmove: &Move) -> bool {

        let from_square = cmove.from_square();
        let to_square   = cmove.to_square();
        let piece       = cmove.piece();
        let capturing   = cmove.is_capture();

        let promotion   = cmove.promotion();
        let double_push = cmove.is_double_push();
        let enpassant   = cmove.is_enpassant();
        let castling    = cmove.is_castling();

        //reset zobrist enpasssant/castling
        if self.enpassant_square != Square::None { self.zobrist_hash ^= ENPASSANT_KEYS[self.enpassant_square as usize] };
        self.zobrist_hash ^= CASTLE_KEYS[self.castling_ability as usize];

        //Update bitboards
        self.bitboards[piece as usize].unset_bit(from_square);
        self.zobrist_hash ^= PIECE_KEYS[piece as usize][from_square as usize];
        self.bitboards[piece as usize].set_bit(to_square);
        self.zobrist_hash ^= PIECE_KEYS[piece as usize][to_square as usize];

        self.all_occupancies.unset_bit(from_square);
        self.all_occupancies.set_bit(to_square);

        //Captures
        if capturing {
            //Enpassant capture
            if enpassant {
                if self.active_player == Color::White {
                    self.bitboards[Piece::BlackPawn as usize].unset_bit(to_square + 8);
                    self.black_occupancies.unset_bit(to_square + 8);
                    self.all_occupancies.unset_bit(to_square + 8);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::BlackPawn as usize][to_square as usize + 8];
                }
                else {
                    self.bitboards[Piece::WhitePawn as usize].unset_bit(to_square - 8);
                    self.white_occupancies.unset_bit(to_square - 8);
                    self.all_occupancies.unset_bit(to_square - 8);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::WhitePawn as usize][to_square as usize - 8];
                }
            } else {
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

                for piece in start..end {
                    if self.bitboards[piece].get_bit(to_square) {
                        self.bitboards[piece].unset_bit(to_square);
                        self.zobrist_hash ^= PIECE_KEYS[piece as usize][to_square as usize];

                        break;
                    }
                }
            }
        }

        //Check check
        if self.is_in_check(self.active_player) {
            return false
        }
        
        //Set/unset color occupancies
        if self.active_player == Color::White {
            self.white_occupancies.unset_bit(from_square);
            self.white_occupancies.set_bit(to_square);
        } else {
            self.black_occupancies.unset_bit(from_square);
            self.black_occupancies.set_bit(to_square);
        }

        //Increment half moves counter if quiet and reset if pawn
        if piece == Piece::WhitePawn as u8 || piece == Piece::BlackPawn as u8 || capturing{
            self.half_moves = 0;
        }
        else {
            self.half_moves += 1;
        }

        //Promotions
        if promotion != Piece::None as u8 {
            //Spawn promoted
            self.bitboards[promotion as usize].set_bit(to_square);

            //Remove pawn
            self.bitboards[piece as usize].unset_bit(to_square);

            //Zobrist update
            self.zobrist_hash ^= PIECE_KEYS[piece as usize][to_square as usize];
            self.zobrist_hash ^= PIECE_KEYS[promotion as usize][to_square as usize];
        }

        //Castling
        else if castling {
            match to_square {
                62 => { //White kingside
                    self.bitboards[Piece::WhiteRook as usize].set_bit_sq(Square::f1);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::WhiteRook as usize][Square::f1 as usize];
                    self.bitboards[Piece::WhiteRook as usize].unset_bit_sq(Square::h1);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::WhiteRook as usize][Square::h1 as usize];
                    self.white_occupancies.set_bit_sq(Square::f1);
                    self.white_occupancies.unset_bit_sq(Square::h1);
                    self.all_occupancies.set_bit_sq(Square::f1);
                    self.all_occupancies.unset_bit_sq(Square::h1);
                }
                58 => { //White queenside
                    self.bitboards[Piece::WhiteRook as usize].set_bit_sq(Square::d1);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::WhiteRook as usize][Square::d1 as usize];
                    self.bitboards[Piece::WhiteRook as usize].unset_bit_sq(Square::a1);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::WhiteRook as usize][Square::a1 as usize];
                    self.white_occupancies.set_bit_sq(Square::d1);
                    self.white_occupancies.unset_bit_sq(Square::a1);
                    self.all_occupancies.set_bit_sq(Square::d1);
                    self.all_occupancies.unset_bit_sq(Square::a1);
                }
                6 => { //Black kingside
                    self.bitboards[Piece::BlackRook as usize].set_bit_sq(Square::f8);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::BlackRook as usize][Square::f8 as usize];
                    self.bitboards[Piece::BlackRook as usize].unset_bit_sq(Square::h8);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::BlackRook as usize][Square::h8 as usize];
                    self.black_occupancies.set_bit_sq(Square::f8);
                    self.black_occupancies.unset_bit_sq(Square::h8);
                    self.all_occupancies.set_bit_sq(Square::f8);
                    self.all_occupancies.unset_bit_sq(Square::h8);
                }
                2 => { //Black queenside
                    self.bitboards[Piece::BlackRook as usize].set_bit_sq(Square::d8);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::BlackRook as usize][Square::d8 as usize];
                    self.bitboards[Piece::BlackRook as usize].unset_bit_sq(Square::a8);
                    self.zobrist_hash ^= PIECE_KEYS[Piece::BlackRook as usize][Square::a8 as usize];
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
                self.enpassant_square = SQUARES[to_square as usize + 8];
                self.zobrist_hash ^= ENPASSANT_KEYS[to_square as usize + 8];
            }
            else {
                self.enpassant_square = SQUARES[to_square as usize - 8];
                self.zobrist_hash ^= ENPASSANT_KEYS[to_square as usize - 8];
            }
        }
        else {
            self.enpassant_square = Square::None
        }

        //Update castling abililties
        self.castling_ability &= CASTLING_RIGHTS[to_square as usize] & CASTLING_RIGHTS[from_square as usize];
        self.zobrist_hash ^= CASTLE_KEYS[self.castling_ability as usize];

        //increment fullmoves & switch player
        if self.active_player == Color::Black {
            self.full_moves += 1;
        }

        self.active_player = opposite_color(self.active_player);
        self.zobrist_hash ^= SIDE_KEY;

        //rep_table.insert(self.zobrist_hash);

        true
    }
}
/*
#[cfg(test)]
mod make_tests {
    use super::*;

    #[test]
    pub fn board_correct_after_move_with_rook() {
        let mut pos = Position::new_from_fen("k7/1R6/8/8/8/8/8/K7 w - - 0 25").unwrap();
        pos.make_move(&Move::new_friendly(Square::b7, Square::e7, Piece::WhiteRook, Piece::None, false, false, false, false));
        assert_eq!(pos.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::b7), false);
        assert_eq!(pos.get_piece_bitboard(Piece::WhiteRook).get_bit_sq(Square::e7), true);
        assert_eq!(pos.white_occupancies.get_bit_sq(Square::b7), false);
        assert_eq!(pos.white_occupancies.get_bit_sq(Square::e7), true);
    }

    #[test]
    pub fn board_correct_after_enpassant_capture() {
        let mut game = Game::new_from_fen("k7/8/8/4Pp2/8/8/8/K7 w - f6 0 25").unwrap();
        
        make_move(&mut game, &Move::new_friendly(Square::e5, Square::f6, Piece::WhitePawn, Piece::None, true, false, true, false));

        //moved to right square
        assert_eq!(game.get_piece_bitboard(Piece::WhitePawn).get_bit_sq(Square::f6), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::f6), true);

        //Captured succesfully
        assert_eq!(game.black_occupancies.get_bit_sq(Square::f5), false);
        assert_eq!(game.get_piece_bitboard(Piece::BlackPawn).get_bit_sq(Square::f5), false);
    }

    #[test]
    pub fn switches_active_player_after_move() {
        let mut game = Game::new_from_start_self();

        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c2);
        make_move(&mut game, &moves[0]);
        assert_eq!(game.active_player, Color::Black);

        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::f7);
        make_move(&mut game, &moves[0]);
        assert_eq!(game.active_player, Color::White);
    }

    #[test]
    pub fn increments_full_moves_correctly() {
        let mut game = Game::new_from_start_self();
        assert_eq!(game.full_moves, 1);

        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::c2);
        make_move(&mut game, &moves[0]);
        assert_eq!(game.full_moves, 1);

        let moves = generate_moves(&mut game, MoveTypes::All).all_from(Square::f7);
        make_move(&mut game, &moves[0]);
        assert_eq!(game.full_moves, 2);
    }

    #[test]
    pub fn resets_half_moves_on_pawn_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 8 1").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::d2, Square::d3, Piece::WhitePawn, Piece::None, false, false, false, false));
        assert!(game.half_moves == 0);
    }

    #[test]
    pub fn resets_half_moves_on_capture() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 8 1").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::f1, Square::b5, Piece::WhiteBishop, Piece::None, true, false, false, false));
        assert!(game.half_moves == 0);
    }

    #[test]
    pub fn increments_half_moves_on_quiet_move() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/1p6/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 8 1").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::f1, Square::c4, Piece::WhiteBishop, Piece::None, false, false, false, false));
        assert!(game.half_moves == 9);
    }

    #[test]
    pub fn pawn_double_push_sets_enpassant_square() {
        let mut game = Game::new_from_start_self();
        make_move(&mut game, &Move::new_friendly(Square::d2, Square::d4, Piece::WhitePawn, Piece::None, false, true, false, false));
        assert_eq!(game.enpassant_square as u8, Square::d3 as u8);
    }

    #[test]
    pub fn enpassant_capture_kills_the_other_pawn() {
        let mut game = Game::new_from_fen("rnbqkbnr/p1pppppp/8/Pp6/8/8/1PPPPPPP/RNBQKBNR w KQkq b6 0 2").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::a5, Square::b6, Piece::WhitePawn, Piece::None, true, false, true, false));
        assert_eq!(game.get_piece_bitboard(Piece::BlackPawn).get_bit_sq(Square::b5), false);
        assert_eq!(game.black_occupancies.get_bit_sq(Square::b5), false);
        assert_eq!(game.all_occupancies.get_bit_sq(Square::b5), false);
    }

    #[test]
    pub fn rooks_correctly_kingside() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 7").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true));
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
        make_move(&mut game, &Move::new_friendly(Square::e1, Square::c1, Piece::WhiteKing, Piece::None, false, false, false, true));
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
        make_move(&mut game, &Move::new_friendly(Square::e1, Square::g1, Piece::WhiteKing, Piece::None, false, false, false, true));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 == 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 == 0);
    }

    #[test]
    pub fn moving_rook_disables_castling_for_that_side() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/8/8/R3K2R w KQkq - 0 2").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::h1, Square::h2, Piece::WhiteRook, Piece::None, false, false, false, false));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 == 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 != 0);
    }

    #[test]
    pub fn capturing_rook_disables_castling_for_that_side() {
        let mut game = Game::new_from_fen("4k3/8/8/8/8/1n6/8/R3K2R b QK - 0 2").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::b3, Square::a1, Piece::BlackKnight, Piece::None, true, false, false, false));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 != 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 == 0);
    }

    #[test]
    pub fn white_pawn_promotes_correctly_on_quietly_reaching_back_row() {
        let mut game = Game::new_from_fen("2n5/1P6/8/8/8/8/8/K6k w - - 0 2").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::b7, Square::b8, Piece::WhitePawn, Piece::WhiteQueen, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteQueen).get_bit_sq(Square::b8), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::b8), true);

    }

    #[test]
    pub fn black_pawn_promotes_correctly_on_quietly_reaching_back_row() {
        let mut game = Game::new_from_fen("K6k/8/8/8/8/8/3p4/8 b - - 0 2").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::d2, Square::d1, Piece::BlackPawn, Piece::BlackRook, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::BlackRook).get_bit_sq(Square::d1), true);
        assert_eq!(game.black_occupancies.get_bit_sq(Square::d1), true);
    }

    #[test]
    pub fn pawn_promotes_correctly_on_back_row_capture() {
        let mut game = Game::new_from_fen("2n5/1P6/8/8/8/8/8/K6k w QK - 0 2").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::b7, Square::c8, Piece::WhitePawn, Piece::WhiteBishop, false, false, false, false));
        assert_eq!(game.get_piece_bitboard(Piece::WhiteBishop).get_bit_sq(Square::c8), true);
        assert_eq!(game.white_occupancies.get_bit_sq(Square::c8), true);
    }

    #[test]
    pub fn moving_the_king_disables_castling() {
        let mut game = Game::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 3 7").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::e1, Square::f1, Piece::WhiteKing, Piece::None, false, false, false, false));
        assert!(game.castling_ability & CastlingAbility::WhiteKingSide as u8 == 0);
        assert!(game.castling_ability & CastlingAbility::WhiteQueenSide as u8 == 0);
    }

    #[test]
    pub fn capturing_bishop_results_in_correct_bitboards() {
        let mut game = Game::new_from_fen("rnbqkbnr/1ppppppp/p6B/8/8/3P4/PPP1PPPP/RN1QKBNR b KQkq - 0 1").unwrap();
        make_move(&mut game, &Move::new_friendly(Square::g8, Square::h6, Piece::BlackPawn, Piece::None, true, false, false, false));
        
        assert_eq!(game.get_piece_bitboard(Piece::WhiteBishop).get_bit_sq(Square::h6), false);
        assert_eq!(game.get_piece_bitboard(Piece::BlackPawn).get_bit_sq(Square::h6), true);

        assert_eq!(game.white_occupancies.get_bit_sq(Square::h6), false);
        assert_eq!(game.black_occupancies.get_bit_sq(Square::h6), true);

        assert_eq!(game.all_occupancies.get_bit_sq(Square::h6), true);
    }
}*/