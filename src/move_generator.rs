use std::mem::MaybeUninit;

use crate::{cmove::Move, position::Position};

#[derive(PartialEq)]
pub enum MoveTypes {
    All,
    CapturesOnly,
}

enum GenPhase {
    Captures,
    NonCaptures,
    Done,
}

/// Lazily generates pseudo legal moves
pub struct MoveGenerator {
    position: Position,

    move_types: MoveTypes,
    phase: GenPhase,
    is_sorting: bool,

    insert_index: usize,
    extract_index: usize,

    move_list: [Move; 100],
}

impl Iterator for MoveGenerator {
    type Item = Move;

    fn next(&mut self) -> Option<Self::Item> {
        // Try generating more moves until some are found, or there are none left
        while self.extract_index == self.insert_index {
            match self.phase {
                GenPhase::Captures => {
                    self.generate_captures();
                    if self.move_types == MoveTypes::All {
                        self.phase = GenPhase::NonCaptures;
                    } else {
                        self.phase = GenPhase::Done;
                    }
                },
                GenPhase::NonCaptures => {
                    self.generate_non_captures();
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
}

impl MoveGenerator {
    pub fn new(position: Position, move_types: MoveTypes, sorted: bool, best: Option<Move>) -> Self {
        let mut generator = Self {
            position,

            move_types,
            phase: GenPhase::Captures,
            is_sorting: sorted,

            insert_index: 0,
            extract_index: 0,

            move_list: [Default::default(); 100], // Check if this is necessary
        };

        if let Some(m) = best {
            generator.move_list[0] = m;
            generator.insert_index += 1;
        }
        
        generator
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

    /// Generate all captures
    fn generate_captures(&mut self) {

    }

    /// Generate all non-captures
    fn generate_non_captures(&mut self) {

    }
}