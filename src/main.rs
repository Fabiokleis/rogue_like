#![allow(dead_code)]
use std::char;
use std::io::{self, Write};

const WIDTH: usize = 20;
const HEIGHT: usize = 10;

type Cell = Vec<Vec<char>>;

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    matrix: Cell,
}


impl Board {
    fn new(width: usize, height: usize) -> Self {

        Board {
            width,
            height,
            matrix: vec![vec!['.'; width]; height]
        }
    }

    fn show(&self) {
        for i in self.matrix.iter() {
            for j in i.iter() {
                print!("{}", j);
            } 
            println!("");
        }
    }
}

/// game loop
fn run(board: &mut Board) {
    let mut quit = false;
    let mut input = String::new();

    while !quit {

        // clear terminal stdout
        std::process::Command::new("clear").status().unwrap();
        board.show();

        // expect a command
        print!("> ");
        io::stdout().flush().expect("cannot flush stdout!");

        // match the possibles commands
        match io::stdin().read_line(&mut input) {
            Ok(_line) => {
                match input.trim() {
                    "q" => quit = true,
                    _ => {}
                }
            },
            Err(error) => println!("error: {error}"),
        }
        
        // clear input string
        input.clear();
    }
}

fn main() {
    let mut board = Board::new(WIDTH, HEIGHT);
    run(&mut board);
}
