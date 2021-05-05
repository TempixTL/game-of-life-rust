# game-of-life-rust

game-of-life-rust is, predictably, an implementation of
[Conway's Game of Life][1] in Rust. More accurately, this project is intended
to help me to learn Rust. So be nice :)

## Installation

Clone to repository to your machine.

```bash
git clone https://github.com/tempixtl/game-of-life-rust.git
```

## Usage

Run the project with Rust's `cargo` tool.

```bash
# Runs a Game of Life simulation with the initial board specified in
# game-of-life-in1.txt for 3 iterations
cargo run input/game-of-life-in1.txt 3
```

[1]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life