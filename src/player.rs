#![allow(dead_code)]
use crate::board::Idxs;

#[derive(Debug, Clone, Copy)]
pub struct Player {
    body: char,
    pos: Idxs,
}

impl Player {
    pub fn new(body: char, pos: Idxs) -> Self {
        Player {
            body,
            pos,
        }
    }
    
    pub fn body(&self) -> char {
        self.body
    }

    pub fn pos(&self) -> Idxs {
        self.pos
    }

    pub fn move_to(&mut self, col: i32, row: i32) {
        self.pos.col = (self.pos.col as i32 + col) as usize;
        self.pos.row = (self.pos.row as i32 + row) as usize;
    }
}
