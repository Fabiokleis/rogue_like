#![allow(dead_code)]
use crate::player::Player;
use std::char;
use std::collections::HashMap;
use std::fmt;
use std::mem;

#[derive(Debug, Clone, Copy)]
pub struct Idxs {
    pub col: usize,
    pub row: usize,
}

#[derive(Clone, Copy)]
pub enum Commands {
    W,
    A,
    S,
    D,
    Clear,
    History,
    Reset,
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Commands::W => write!(f, "W"),
            Commands::A => write!(f, "A"),
            Commands::S => write!(f, "S"),
            Commands::D => write!(f, "D"),
            Commands::Clear => write!(f, "CLEAR"),
            Commands::Reset => write!(f, "RESET"),
            Commands::History => write!(f, "HISTORY"),
        }
    }
}

#[derive(Clone)]
pub struct Board {
    width: usize,
    height: usize,
    pub commands: HashMap<String, Commands>,
    history: Vec<String>,
    matrix: Vec<Vec<char>>,
    player: Player,
}

impl Board {
    /// create a new empty Board with width and height consts
    pub fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            commands: HashMap::from([
                (String::from("w"), Commands::W),
                (String::from("a"), Commands::A),
                (String::from("s"), Commands::S),
                (String::from("d"), Commands::D),
                (String::from("clear"), Commands::Clear),
                (String::from("history"), Commands::History),
                (String::from("reset"), Commands::Reset),
            ]),
            history: Vec::<String>::new(),
            matrix: vec![vec!['.'; width]; height],
            player: Player::new('@', Idxs { col: width/2, row: height/2 }),
        }
    }

    /// show the current state of matrix
    pub fn show(&self) {
        for i in self.matrix.iter() {
            for j in i.iter() {
                print!("{}", j);
            }
            println!("");
        }
    }

    /// create player at board matrix
    pub fn load_player(&mut self) {
        let idx = self.player.pos();
        self.matrix[idx.row][idx.col] = self.player.body();
    }

    /// replace the position of the player body at board matrix
    pub fn update_player(&mut self, dir: (i32, i32)) {
        let idx = self.player.pos();
        let unknown_body = mem::replace(&mut self.matrix[idx.row][idx.col], self.player.body());

        // replace last position of the player
        let last_idx = Idxs {
            col: (idx.col as i32 - dir.0) as usize,
            row: (idx.row as i32 - dir.1) as usize,
        };
        self.matrix[last_idx.row][last_idx.col] = unknown_body;
    }

    /// capture and match every command type
    pub fn handle_command(&mut self, cmd: String) {
        let cmd = cmd.trim().to_string();
        match self.commands.get(&cmd) {
            Some(Commands::W) => {
                if self.player.pos().row > 0 {
                    let dir = (0, -1);
                    self.player.move_to(dir.0, dir.1);
                    self.update_player(dir);
                }
            }
            Some(Commands::A) => {
                if self.player.pos().col > 0 {
                    let dir = (-1, 0);
                    self.player.move_to(dir.0, dir.1);
                    self.update_player(dir);
                }
            }
            Some(Commands::S) => {
                if self.player.pos().row < self.height - 1 {
                    let dir = (0, 1);
                    self.player.move_to(dir.0, dir.1);
                    self.update_player(dir);
                }
            }
            Some(Commands::D) => {
                if self.player.pos().col < self.width - 1 {
                    let dir = (1, 0);
                    self.player.move_to(dir.0, dir.1);
                    self.update_player(dir);
                }
            }
            Some(Commands::History) => self.show_history(),
            Some(Commands::Clear) => self.clear_term(),
            _ => {}
        }
    }

    /// pushs a new command to the history vec
    pub fn push_history(&mut self, cmd: String) {
        self.history.push(cmd);
    }

    /// show history vector elements
    pub fn show_history(&self) {
        for (i, cmd) in self.history.iter().enumerate() {
            println!("{} {}", i, cmd);
        }
    }

    /// clear terminal stdout
    pub fn clear_term(&self) {
        std::process::Command::new("clear")
            .status()
            .expect("cannot clear terminal!");
    }

    /// set every cell of the matrix to empty char
    pub fn clear_board(&mut self) {
        for i in self.matrix.iter_mut() {
            for j in i.iter_mut() {
                *j = '.';
            }
        }
    }
}
