use std::io::{self, Write};

mod player;
mod board;
use board::Board;
const WIDTH: usize = 30;
const HEIGHT: usize = 15;

/// game loop
fn run(board: &mut Board) {
    board.load_player();
    let mut input = String::new();

    'running: loop {
        board.show();
        print!("> ");
        io::stdout().flush().expect("cannot flush stdout!");

        match io::stdin().read_line(&mut input) {
            Ok(_line) => {
                board.push_history(input.trim().to_string());
                match input.trim() {
                    "q" | "quit" => break 'running,
                    _ => {
                        // match the possible commands
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
