use rayon::prelude::*;

use super::*;
pub struct MoveList {
    moves: [Option<Move>; 256],
    count: usize
}

const MOVE_LIST_SIZE: usize = 256;

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [None; MOVE_LIST_SIZE],
            count: 0
        }
    }
    
    pub fn add_move(&mut self, cmove: Move) {
        self.moves[self.count] = Some(cmove);
        self.count += 1;
    }

    #[cfg(test)]
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
            None => unreachable!("Index: {} was out of bounds. Length is only: {}", index, self.len()),
        }
    }

    pub fn legal_values(&self, game: &Game) -> Vec<Move> {
        let res: Vec<Move> = self.moves.iter().take(self.count).map(|m| m.unwrap()).filter(|m| is_legal(game, m)).collect();

        res
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = Move> + 'a {
        self.moves.iter().take(self.count).map(|m| m.expect("Move is None here!"))
    }

    pub fn par_iter<'a>(&'a self) -> impl ParallelIterator<Item = Move> + 'a {
        self.moves.par_iter().take(self.count).map(|m| m.expect("Move is None here!"))
    }

    pub fn count_legal(&self, game: &mut Game) -> usize {
        self.par_iter().filter(|m| is_legal(game, m)).count()
    }

    ///Sorts the moves by their score_move() value with insertion sort
    pub fn sort_moves(&mut self, game: &Game, envir: &mut SearchEnv) {
        let mut scores = [0; MOVE_LIST_SIZE];
        for i in 0..self.count {
            scores[i] = score_move(&game, self.moves[i].unwrap(), envir)
        }
        
        //Unoptimized!
        for i in 0..self.count {
            for j in (i + 1)..self.count {
                if scores[j] > scores[i] {
                    let score = scores[j];
                    scores[j] = scores[i];
                    scores[i] = score;

                    let mov = self.moves[j];
                    self.moves[j] = self.moves[i];
                    self.moves[i] = mov;
                }
            }
        }
    }

    #[cfg(test)]
    pub fn contains(&self, cmove: &Move) -> bool {
        return self.moves.contains(&Some(*cmove))
    }

    #[cfg(test)]
    pub fn all_from(&self, square: Square) -> Vec<Move> {
        let mut v: Vec<Move> = Vec::new();

        self.moves.iter().for_each(|f| if f.is_some() && f.unwrap().from_square() == square as u8 { v.push(f.unwrap()) });

        return v
    }
}

trait MoveContainer<'a> {
    type ItemIterator: Iterator<Item=&'a u8>;

    fn items(&'a self) -> Self::ItemIterator;
}