use std::vec;

use crate::Direction;

pub struct Piece {
    content: char,
    shape: Vec<Vec<u8>>
}

impl Piece {
    pub fn new_o() -> Piece {
        Piece {
            content: 'o',
            shape: vec![
                        vec![1,1],
                        vec![1,1]]
        }
    }
    pub fn new_l() -> Piece {
        Piece { content: 'l', 
                shape: vec![vec![0, 0, 1],
                            vec![1, 1, 1],
                            vec![0, 0, 0]] 
        }
    }

    pub fn new_j() -> Piece {
        Piece{
            content: 'j',
            shape: vec![vec![1, 0, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }
    pub fn new_t() -> Piece {
        Piece{
            content: 't',
            shape: vec![vec![0, 1, 0],
                        vec![1, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_s() -> Piece {
        Piece{
            content: 's',
            shape: vec![vec![0, 1, 1],
                        vec![1, 1, 0],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_z() -> Piece {
        Piece{
            content: 'z',
            shape: vec![vec![1, 1, 0],
                        vec![0, 1, 1],
                        vec![0, 0, 0]]
        }
    }

    pub fn new_i() -> Piece {
        Piece{
            content: 'i',
            shape: vec![vec![0, 0, 0, 0],
                        vec![1, 1, 1, 1],
                        vec![0, 0, 0, 0],
                        vec![0, 0, 0, 0]]
        }
    }

    fn rotate(&mut self, direction: Direction) {
        let size = self.shape.len();
        for row in 0..size / 2 {
            for col in row..(size - row - 1) {
                let t = self.shape[row][col];

                match direction {
                    Direction::Left => {
                        self.shape[row][col] = self.shape[col][size - row - 1];
                        self.shape[col][size - row - 1] = self.shape[size - row - 1][size - col - 1];
                        self.shape[size - row - 1][size - col - 1] = self.shape[size - col - 1][row];
                        self.shape[size - col - 1][row] = t;
                    }
                }
            }
        }
    }
}

