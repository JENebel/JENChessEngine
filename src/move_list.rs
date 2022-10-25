use rayon::prelude::*;

use super::*;
pub struct MoveList {
    moves: [Move; MOVE_LIST_SIZE],
    count: usize
}

const MOVE_LIST_SIZE: usize = 256;

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [NULL_MOVE; MOVE_LIST_SIZE],
            count: 0
        }
    }
    
    #[inline(always)]
    pub fn add_move(&mut self, cmove: Move) {
        self.moves[self.count] = cmove;
        self.count += 1;
    }

    #[cfg(test)]
    pub fn print(&self) {
        println!("Move count: {}", self.count);
        for i in 0..self.count {
            self.moves[i].print();
        }
        println!();
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn get(&self, index: usize) -> Move {
        self.moves[index]
    }

    pub fn legal_values(&self, game: &Game) -> Vec<Move> {
        let res: Vec<Move> = self.moves.iter().take(self.count).filter(|m| is_legal(game, m)).map(|m| *m).collect();

        res
    }

    pub fn iter(&self) -> std::iter::Take<std::slice::Iter<'_, cmove::Move>> {
        self.moves.iter().take(self.count)
    }

    pub fn par_iter(&self) -> rayon::iter::Take<rayon::slice::Iter<'_, cmove::Move>> {
        self.moves.par_iter().take(self.count)
    }

    pub fn bulk_count(&self, game: &mut Game) -> usize {
        let mut res = 0;
        for i in 0..self.len() {
            if is_legal(game, &self.moves[i]){
                res += 1
            }
        }
        res
    }

    #[inline(always)]
    ///Sorts the moves by their score_move() value with insertion sort
    pub fn sort_moves(&mut self, game: &Game, envir: &mut SearchEnv) {
        let mut scores = [0; MOVE_LIST_SIZE];
        for i in 0..self.count {
            scores[i] = score_move(&game, self.moves[i], envir)
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
        return self.moves.contains(cmove)
    }

    #[cfg(test)]
    pub fn all_from(&self, square: Square) -> Vec<Move> {
        let mut v: Vec<Move> = Vec::new();

        self.moves.iter().for_each(|f| if f.from_square() == square as u8 { v.push(*f) });

        return v
    }
}