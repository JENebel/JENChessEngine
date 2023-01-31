use crate::{position::Position, definitions::*, bitboard::Bitboard};

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
            match self.phase {
                GenPhase::Interesting => {
                    self.generate_interesting_moves(pos);
                    if self.move_types == MoveTypes::All {
                        self.phase = GenPhase::Quiet;
                    } else {
                        self.phase = GenPhase::Done;
                    }
                },
                GenPhase::Quiet => {
                    self.generate_quiet_moves(pos);
                    self.phase = GenPhase::Done;
                },
                GenPhase::Done => return None,
            }
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

    fn insert_and_score(&mut self, new_move: &mut Move) {
        if self.is_sorting {
            Self::score_move(new_move) // Maybe handle scoring different
        }
        self.insert(*new_move)
    }

    #[inline(always)]
    fn insert(&mut self, new_move: Move) {
        self.move_list[self.insert_index] = new_move;
        self.insert_index += 1;
    }

    pub fn score_move(m: &mut Move) {
        m.score = 10; // Fake it. Should probably be moved to seperate place
    }


    /// Generate interesting moves
    /// 
    /// Check evasions, captures, promotions
    fn generate_interesting_moves(&mut self, pos: &Position) {
        if pos.is_in_check() {
            self.generate_check_evasions(pos);
            return
        }
    }

    /// Generate check evasions
    fn generate_check_evasions(&mut self, pos: &Position) {

        ///////

        // Should not generate more moves as this generates all possible
        self.phase = GenPhase::Done
    }

    /// Generate quiet moves
    fn generate_quiet_moves(&mut self, pos: &Position) {

    }

    /// Generates all legal king moves
    fn generate_king_moves() {
        
    }
}