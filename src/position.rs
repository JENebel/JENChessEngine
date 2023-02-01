use crate::{key_constants::*, bitboard::*, attack_tables::*, definitions::*};

use Color::*;
use Piece::*;
use PieceType::*;

#[derive(Clone, Copy)]
pub struct Position {
    bitboards: [Bitboard; 12],

    //3 occupancy bitboards
    pub white_occupancies: Bitboard,
    pub black_occupancies: Bitboard,
    pub all_occupancies:   Bitboard,

    // Pieces currently xraying the king
    xray_hv_white:   Bitboard, // White pieces xraying the black king
    xray_hv_black:   Bitboard, // Black pieces xraying the white king
    xray_diag_white: Bitboard, // White pieces xraying the black king
    xray_diag_black: Bitboard, // Black pieces xraying the white king

    // Pieces currently pinned
    pinned_hv_white:   Bitboard,
    pinned_diag_white: Bitboard,
    pinned_hv_black:   Bitboard,
    pinned_diag_black: Bitboard,

    // The pin masks for the current position.
    // This is the 
    pub pin_mask_white: Bitboard,

    /// The check mask for the current position
    /// These are the squares where checking pieces are, and the paths between the king and sliding checking pieces.\
    /// If not in check it is entirely 1's.
    pub check_mask: Bitboard,

    pub active_player: Color,
    pub enpassant_square: Square,
    pub castling_ability: u8,

    pub full_moves: u16,
    pub half_moves: u8,
    pub zobrist_hash: u64,

    // Repetition table should be included for make/unmake
}

impl Position {
    pub fn pretty_print(&self) {
        println!("\n  ┌────┬────┬────┬────┬────┬────┬────┬────┐");
        for y in 0..8 {
            print!("{} │", format!("{}", 8-y ).as_str());
            for x in 0..8 {
                let piece_index = (0..11).find(|i| self.bitboards[*i].get_bit(8*y+x)).unwrap();
                print!(" {}{} ", PIECE_STRINGS[piece_index], if piece_index < 6 {" "} else {"."});
                if x != 7 { print!("│") };
            }
            println!("│");
            if y != 7 { println!("  ├────┼────┼────┼────┼────┼────┼────┼────┤")};
        }
        println!("  └────┴────┴────┴────┴────┴────┴────┴────┘");
        println!("    a    b    c    d    e    f    g    h\n");

          print!("   Active:     {}",     if self.active_player == Color::White { "White" } else { "Black" });
        println!("\tFull moves: {}",      self.full_moves);
          print!("   Enpassant:  {}",     SQUARE_STRINGS[self.enpassant_square as usize]);
        println!("\tHalf moves: {}",      self.half_moves);
          print!("   Castling:   {}  ",   self.castling_ability_string());
        println!("\tZobrist:   {:#0x}\n", self.zobrist_hash);
    }

    fn castling_ability_string(&self) -> String {
        let mut result = String::new();
        if self.castling_ability & CastlingAbility::WhiteKingSide   as u8 != 0  { result += "K" }
        if self.castling_ability & CastlingAbility::WhiteQueenSide  as u8 != 0  { result += "Q" }
        if self.castling_ability & CastlingAbility::BlackKingSide   as u8 != 0  { result += "k" }
        if self.castling_ability & CastlingAbility::BlackQueenSide  as u8 != 0  { result += "q" }
        result
    }

    pub fn new_from_start_pos() -> Self {
        Position::new_from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }

    pub fn new_from_fen(input: &str) -> Option<Self> {
        let fen = input.trim();
        let mut split = fen.split(' ').peekable();

        let mut bitboards: [Bitboard; 12] =  [Default::default(); 12];
        let mut white_occupancies: Bitboard = Default::default();
        let mut black_occupancies: Bitboard = Default::default();
        let mut all_occupancies:   Bitboard = Default::default();

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

            xray_hv_white:     Bitboard::default(),
            xray_hv_black:     Bitboard::default(),
            xray_diag_white:   Bitboard::default(),
            xray_diag_black:   Bitboard::default(),
            pinned_hv_white:   Bitboard::default(),
            pinned_diag_white: Bitboard::default(),
            pinned_hv_black:   Bitboard::default(),
            pinned_diag_black: Bitboard::default(),
            check_mask:        Bitboard::default(),
            pin_mask_white:        Bitboard::default(),

            active_player,
            castling_ability,
            enpassant_square,

            full_moves,
            half_moves,
            zobrist_hash: u64::default(),
        };
        
        pos.generate_check_mask();
        pos.generate_xrays();
        pos.generate_pins(White);
        pos.generate_pins(Black);
        pos.generate_zobrist_hash();

        Some(pos)
    }

    /// Calculates the pinned pieces bb
    fn calc_pinned(&self) -> Bitboard {
        let mut possibly_pinned = self.get_color_bitboard(self.active_player);
        possibly_pinned.unset_bit(self.king_position(self.active_player)); // King cannot be pinned

        Bitboard::default()
    }

    #[inline(always)]
    pub fn get_piece_color_bitboard(&self, piece_type: PieceType, color: Color) -> Bitboard {
        self.bitboards[if color == Color::Black { piece_type as usize + 6 } else { piece_type as usize }]
    }

    #[inline(always)]
    pub fn get_piece_bitboard(&self, piece: Piece) -> Bitboard {
        self.bitboards[piece as usize]
    }

    #[inline(always)]
    pub fn get_bitboard(&self, piece_index: usize) -> Bitboard {
        self.bitboards[piece_index]
    }

    #[inline(always)]
    pub fn get_color_bitboard(&self, color: Color) -> Bitboard {
        if color == Color::White {
            self.white_occupancies
        } else {
            self.black_occupancies
        }
    }

    #[inline(always)]
    pub fn get_hv_xrays(&self, color: Color) -> Bitboard {
        if color == Color::White {
            self.xray_hv_white
        } else {
            self.xray_hv_black
        }
    }

    #[inline(always)]
    pub fn get_diag_xrays(&self, color: Color) -> Bitboard {
        if color == Color::White {
            self.xray_diag_white
        } else {
            self.xray_diag_black
        }
    }

    #[inline(always)]
    pub fn get_hv_pinned(&self, color: Color) -> Bitboard {
        if color == Color::White {
            self.pinned_hv_white
        } else {
            self.pinned_hv_black
        }
    }

    #[inline(always)]
    pub fn get_diag_pinned(&self, color: Color) -> Bitboard {
        if color == Color::White {
            self.pinned_diag_white
        } else {
            self.pinned_diag_black
        }
    }

    #[inline(always)]
    pub fn set_hv_pinned(&mut self, color: Color, square: u8) {
        if color == Color::White {
            self.pinned_hv_white.set_bit(square)
        } else {
            self.pinned_hv_black.set_bit(square)
        }
    }

    #[inline(always)]
    pub fn set_diag_pinned(&mut self, color: Color, square: u8) {
        if color == Color::White {
            self.pinned_diag_white.set_bit(square)
        } else {
            self.pinned_diag_black.set_bit(square)
        }
    }

    /// Creates a zobrist hash from scratch for the current position
    fn generate_zobrist_hash(&mut self) {
        let mut hash = 0;

        for piece in 0..12 {
            let mut bb = self.bitboards[piece];
            while let Some(square) = bb.extract_bit() {
                hash ^= PIECE_KEYS[piece][square as usize];
            }
        }

        hash ^= CASTLE_KEYS[self.castling_ability as usize];
        
        if self.active_player == Black {
            hash ^= SIDE_KEY;
        }

        if self.enpassant_square != Square::None {
            hash ^= ENPASSANT_KEYS[self.enpassant_square as usize];
        }

        self.zobrist_hash = hash
    }

    #[inline(always)]
    /// Indicates whether a square is attacked
    pub fn is_square_attacked(&self, square: u8, by_color: Color) -> bool {
        get_pawn_attack_table   (square, opposite_color(by_color)) .and(self.get_piece_color_bitboard(Pawn,   by_color)).is_not_empty() ||
        get_knight_attack_table (square)                           .and(self.get_piece_color_bitboard(Knight, by_color)).is_not_empty() ||
        get_king_attack_table   (square)                           .and(self.get_piece_color_bitboard(King,   by_color)).is_not_empty() ||
        get_rook_attack_table   (square, self.all_occupancies)     .and(self.get_piece_color_bitboard(Rook,   by_color)).is_not_empty() ||
        get_bishop_attack_table (square, self.all_occupancies)     .and(self.get_piece_color_bitboard(Bishop, by_color)).is_not_empty() ||
        get_queen_attack_table  (square, self.all_occupancies)     .and(self.get_piece_color_bitboard(Queen,  by_color)).is_not_empty()
    }

    /// Indicates whether the currently active player is in check
    #[inline(always)]
    pub fn is_in_check(&self) -> bool {
        self.check_mask.not().is_not_empty()
    }

    /// Gets the position of the king of the given color
    #[inline(always)]
    pub fn king_position(&self, color: Color) -> u8 {
        self.get_piece_color_bitboard(King, color).least_significant()
    }

    /// Gets the check mask for the current player.
    fn generate_check_mask(&mut self) {
        let mut mask = Bitboard::from_u64(u64::MAX);

        let king_pos = self.king_position(self.active_player);
        let opponent = opposite_color(self.active_player);

       // Leapers
        mask = mask.xor(
        get_pawn_attack_table   (king_pos, self.active_player).and(self.get_piece_color_bitboard(Pawn, opponent)).or(
        get_knight_attack_table (king_pos).and(self.get_piece_color_bitboard(Knight, opponent))));
        
       // Sliders
        let hv_rays_from_king = get_rook_attack_table(king_pos, self.all_occupancies);
        let diag_rays_from_king = get_bishop_attack_table(king_pos, self.all_occupancies);

        // Rooks
        {
            let mut rooks_checking = hv_rays_from_king.and(self.get_piece_color_bitboard(Rook, opponent));
            while let Some(rook) = rooks_checking.extract_bit() {
                mask = mask.xor(get_rook_attack_table(rook, self.all_occupancies).and(hv_rays_from_king));
                mask.unset_bit(rook)
            }
        }

        // Bishops
        {
            let mut bishops_checking = diag_rays_from_king.and(self.get_piece_color_bitboard(Bishop, opponent));
            while let Some(bishop) = bishops_checking.extract_bit() {
                mask = mask.xor(get_bishop_attack_table(bishop, self.all_occupancies).and(diag_rays_from_king));
                mask.unset_bit(bishop)
            }
        }

        // Queen
        {
            let mut queens_checking = (diag_rays_from_king.or(hv_rays_from_king)).and(self.get_piece_color_bitboard(Queen, opponent));
            while let Some(queen) = queens_checking.extract_bit() {
                // Determine if the check is on the diagonal or the hv rays
                let queen_hv_rays = get_rook_attack_table(queen, self.all_occupancies);
                if queen_hv_rays.get_bit(king_pos) {
                    // We know it is on the HV ray
                    mask = mask.xor(queen_hv_rays.and(hv_rays_from_king));
                } else {
                    // Must be on diagonal
                    mask = mask.xor(get_bishop_attack_table(queen, self.all_occupancies).and(diag_rays_from_king));
                }
                mask.unset_bit(queen)
            }
        }

        self.check_mask = mask.not()
    }

    fn generate_xrays(&mut self) {
        self.xray_hv_white = get_rook_attack_table(self.king_position(Black), Bitboard::from_u64(0)).and( (self.get_piece_bitboard(WhiteRook)).or(self.get_piece_bitboard(WhiteQueen)) );
        self.xray_hv_white = get_rook_attack_table(self.king_position(White), Bitboard::from_u64(0)).and( (self.get_piece_bitboard(BlackRook)).or(self.get_piece_bitboard(BlackQueen)) );

        self.xray_diag_white = get_bishop_attack_table(self.king_position(Black), Bitboard::from_u64(0)).and( (self.get_piece_bitboard(WhiteBishop)).or(self.get_piece_bitboard(WhiteQueen)) );
        self.xray_diag_white = get_bishop_attack_table(self.king_position(White), Bitboard::from_u64(0)).and( (self.get_piece_bitboard(BlackBishop)).or(self.get_piece_bitboard(BlackQueen)) );
    }

    fn generate_pins(&mut self, color: Color) {
        let king_pos = self.king_position(color);
        let opponent = opposite_color(color);

        for bb in color as usize..color as usize + 5 { // Iterates from pawn to queen
            let mut board = self.get_bitboard(bb);
            while let Some(square) = board.extract_bit() {
                // Basically works by checking if both the own king and an opponent xraying piece is in line of sight

                // HV pins
                let hv_from_sq = get_rook_attack_table(square, self.all_occupancies);
                if hv_from_sq.get_bit(king_pos) && hv_from_sq.and(self.get_hv_xrays(opponent)).is_not_empty() {
                    // Is pinned
                    self.set_hv_pinned(color, square);
                    continue;
                }

                // Diagonal pins
                let diag_from_sq = get_bishop_attack_table(square, self.all_occupancies);
                if diag_from_sq.get_bit(king_pos) && diag_from_sq.and(self.get_diag_xrays(opponent)).is_not_empty() {
                    // Is pinned
                    self.set_diag_pinned(color, square);
                }
            }
        }
    }
}