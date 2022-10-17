use std::iter::Map;

use super::*;
pub struct MoveList {
    moves: [Option<Move>; 256],
    count: usize
}

impl MoveList {
    pub fn new(game: &Game) -> Self {
        Self {
            moves: [None; 256],
            count: 0
        }
    }
    
    pub fn add_move(&mut self, cmove: Move) {
        self.moves[self.count] = Some(cmove);
        self.count += 1;
    }

    pub fn print(&self) {
        println!("Move count: {}", self.count);
        for i in 0..self.count {
            self.moves[i].unwrap().print();
        }
        println!();
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get(&self, index: usize) -> &Move {
        match &self.moves[index] {
            Some(m) => &m,
            None => unreachable!(),
        }
    }
}

#[cfg(test)]
impl MoveList {
    pub fn contains(&self, cmove: &Move) -> bool {
        return self.moves.contains(&Some(*cmove))
    }

    pub fn all_from(&self, square: Square) -> Vec<Move> {
        let mut v: Vec<Move> = Vec::new();

        self.moves.iter().for_each(|f| if f.is_some() && f.unwrap().from_square() == square as u8 { v.push(f.unwrap()) });

        return v
    }
}