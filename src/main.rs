#![allow(dead_code)]
use std::char;
use std::collections::HashMap;
use std::io::{self, Write};
use std::fmt;

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

#[derive(Clone, Copy)]
enum Commands {
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
struct Board {
    width: usize,
    height: usize,
    commands: HashMap<String, Commands>,
    history: Vec<String>,
    matrix: Vec<Vec<char>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            width,
            height,
            commands: HashMap::<String, Commands>::new(),
            history: Vec::<String>::new(),
            matrix: vec![vec!['.'; width]; height],
        }
    }

    // show the current state of matrix
    fn show(&self) {
        for i in self.matrix.iter() {
            for j in i.iter() {
                print!("{}", j);
            }
            println!("");
        }
    }

    /// capture and match every command type
    fn handle_command(&mut self, cmd: String) {
        let cmd = cmd.trim().to_string();
        self.history.push(cmd.clone());
        match self.commands.get(&cmd) {
            Some(Commands::W) => todo!(),
            Some(Commands::A) => todo!(),
            Some(Commands::S) => todo!(),
            Some(Commands::D) => todo!(),
            Some(Commands::History) => self.show_history(),
            Some(Commands::Clear) => self.clear_term(),
            _ => {}
        }
    }

    /// show history vector elements
    fn show_history(&self) {
        for (i, cmd) in self.history.iter().enumerate() {
            println!("{} {}", i, cmd);
        }
    }

    /// clear terminal stdout
    fn clear_term(&self) {
        std::process::Command::new("clear").status().expect("cannot clear terminal!");
    }

    fn populate_commands(&mut self) {
        self.commands = HashMap::from([
            (String::from("w"), Commands::W),
            (String::from("a"), Commands::A),
            (String::from("s"), Commands::S),
            (String::from("d"), Commands::D),
            (String::from("clear"), Commands::Clear),
            (String::from("history"), Commands::History),
            (String::from("reset"), Commands::Reset),
        ]);
    }

    fn clear_board(&mut self) {
        for i in self.matrix.iter_mut() {
            for j in i.iter_mut() {
                *j = '.';
            }
        }
    }
}

/// game loop
fn run(board: &mut Board) {

    board.populate_commands();
    let mut input = String::new();

    'running: loop {
        
        board.show();
        print!("> ");
        io::stdout().flush().expect("cannot flush stdout!");

        // match the possible commands
        match io::stdin().read_line(&mut input) {
            Ok(_line) => {
                match input.trim() {
                    "q" | "quit" => break 'running,
                    _ => {
                        if board.commands.contains_key(input.trim()) {
                            board.handle_command(input.clone());
                        }
                    },
                } 
            },
            Err(_) => {}
        }

        // clear input string
        input.clear();
    }
}

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);
    run(&mut board);
}
