# rogue_like

Rogue like game in Rust

## Quick Start
```console
cargo run
```

## Commands
This game has a list of commands that can execute at runtime.

You can interact with player and the 'command prompt' by typing in STDIN using:
```Rust
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
```

## License
MIT
