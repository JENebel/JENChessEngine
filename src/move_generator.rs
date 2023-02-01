use crate::{position::Position, definitions::*, bitboard::Bitboard};
use PieceType::*;

// Macros to expand const generics for move generation
macro_rules! generate_moves_match_has_enpassant {
    ($move_gen: expr, $pos: expr, $is_white: expr, $phase: expr, $is_sorting: expr) => {
        match $pos.enpassant_square.is_some() {
            true =>  $move_gen.generate_moves::<$is_white, $phase, $is_sorting, true>($pos),
            false => $move_gen.generate_moves::<$is_white, $phase, $is_sorting, false>($pos),
        }
    };
}
macro_rules! generate_moves_match_is_sorting {
    ($move_gen: expr, $pos: expr, $is_white: expr, $phase: expr) => {
        match $move_gen.is_sorting {
            true =>  generate_moves_match_has_enpassant!($move_gen, $pos, $is_white, $phase, true),
            false => generate_moves_match_has_enpassant!($move_gen, $pos, $is_white, $phase, true),
        }
    };
}
macro_rules! generate_moves_match_phase {
    ($move_gen: expr, $pos: expr, $is_white: expr) => {
        match $move_gen.phase {
            GenPhase::Interesting => generate_moves_match_is_sorting!($move_gen, $pos, $is_white, 0),
            GenPhase::Quiet =>       generate_moves_match_is_sorting!($move_gen, $pos, $is_white, 1),
            GenPhase::Done =>        return None
        }
    };
}
macro_rules! generate_moves {
    ($move_gen: expr, $pos: expr) => {
        // Match color
        match $pos.active_color {
            Color::White => generate_moves_match_phase!($move_gen, $pos, true),
            Color::Black => generate_moves_match_phase!($move_gen, $pos, false),
        }
    };
}

pub struct MoveGenerator {
    move_types: MoveTypes,
    phase: GenPhase,
    is_sorting: bool,

    insert_index: usize,
    extract_index: usize,

    move_list: [Move; 100],
}

impl MoveGenerator {
    /// Creates a new move generator
    pub fn new(position: &Position, move_types: MoveTypes, sort: bool) -> Self {
        Self {
            move_types,
            phase: Default::default(),
            is_sorting: sort,
            
            insert_index: 0,
            extract_index: 0,

            move_list: [Default::default(); 100], // Check if this is necessary
        }
    }

    pub fn add_pv_move(&mut self, pv_move: Move) {
        self.insert(pv_move)
    }

    /// Gets the next move in the position
    pub fn next_move(&mut self, pos: &Position) -> Option<Move> {
        // Try generating more moves until some are found, or there are none left
        while self.extract_index == self.insert_index {
            generate_moves!(self, pos)
        }

        if self.is_sorting {
            Some(self.extract_best())
        } else {
            Some(self.extract_first())
        }
    }

    /// Extracts the best move in the list
    fn extract_best(&mut self) -> Move {
        let mut best_index = self.extract_index;

        for i in self.extract_index..self.insert_index {
            let best_score = self.move_list[best_index].score;
            let score = self.move_list[i].score;

            if score > best_score {
                best_index = i
            }
        }

        self.move_list.swap(self.extract_index, best_index);

        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;
        extracted
    }

    /// Extract the first item in the list
    fn extract_first(&mut self) -> Move {
        let extracted = self.move_list[self.extract_index];
        self.extract_index += 1;
        extracted
    }

    #[inline(always)]
    fn insert_and_score<const ConstSort: bool>(&mut self, new_move: &mut Move) {
        if ConstSort {
            Self::score_move(new_move) // Maybe handle scoring different
        }
        self.insert(*new_move)
    }

    #[inline(always)]
    fn insert(&mut self, new_move: Move) {
        self.move_list[self.insert_index] = new_move;
        self.insert_index += 1;
    }

    fn score_move(m: &mut Move) {
        m.score = 10; // Fake it. Should probably be moved to seperate place
    }

    /// Generate interesting moves
    /// 
    /// Check evasions, captures, promotions
    fn generate_moves<const IS_WHITE: bool, const PHASE: u8, const IS_SORTING: bool, const HAS_ENPASSANT: bool>(&mut self, pos: &Position) {
        let active_color = if IS_WHITE { Color::White } else {Color::Black };
        
        // Go straight to check evasions if in check
        if pos.is_in_check(active_color) {
            self.generate_check_evasions::<IS_WHITE>(pos);
            return
        }
        
        let gen_phase: GenPhase = unsafe { std::mem::transmute(PHASE) }; // Always a legal phase!
        let is_sorting = IS_SORTING;
        let has_enpassant = HAS_ENPASSANT;

        let king_pos = pos.king_position(active_color);

        let king_file = FILE_MASKS[king_pos as usize];
        let king_rank = RANK_MASKS[king_pos as usize];

        let opp_hv_sliders = pos.get_piece_color_bitboard(Rook, active_color);

        //while let Some()

        match active_color {
            Color::White => { },
            Color::Black => { },
        }

        match gen_phase {
            GenPhase::Interesting => {
                if self.move_types == MoveTypes::All {
                    self.phase = GenPhase::Quiet
                } else {
                    self.phase = GenPhase::Done
                }
            },
            GenPhase::Quiet => { self.phase = GenPhase::Done },
            _ => { }
        }
    }

    fn get_hv_slider_mask() {

    }

    /// Generate check evasions
    fn generate_check_evasions<const ConstColor: bool>(&mut self, pos: &Position) {
        ///////

        // Should not generate more moves as this generates all possible
        self.phase = GenPhase::Done
    }

    /// Generates all legal king moves
    fn generate_king_moves() {
        
    }
}

#[test]
pub fn test() {
    let pos = Position::new_from_start_pos();
    let mut gene = MoveGenerator::new(&pos, MoveTypes::All, false);

    println!("{:?}", gene.next_move(&pos));
}