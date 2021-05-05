use std::fmt;
use std::fmt::Formatter;
use std::ops;
use crate::cfg::Config;

pub fn run(config: Config) {
    let Config { starting_board, steps } = config;
    match Board::new(starting_board) {
        Some(board) => {
            let mut current_board = board;
            for iteration in 0..=steps {
                if iteration == 0 {
                    println!("Initial board:");
                } else {
                    println!("Board after step {}:", iteration)
                }
                println!("{}", current_board);
                current_board = current_board.step();
            }
        },
        None => eprintln!("Unable to parse board."),
    }
}

#[derive(Copy, Clone, PartialEq)]
enum Cell {
    Alive,
    Dead,
}

/// A Board which stores a grid of [`Cell`] in row-major order.
struct Board {
    grid: Vec<Vec<Cell>>,
    width: usize,
    height: usize,
}

impl Board {
    pub fn new(str: String) -> Option<Board> {
        let mut str_iter = str.lines();

        let grid_size_str = str_iter.next()?;
        let grid_size = grid_size_str.parse::<usize>().ok()?;
        let grid: Vec<Vec<Cell>> = str_iter.map(|grid_line|
            grid_line.split(" ").map(|grid_char| match grid_char {
                "1" => Cell::Alive,
                _   => Cell::Dead,
            }).collect()
        ).collect();

        if grid_size == grid.len() && grid.iter().all(|grid_line| grid_size == grid_line.len()) {
            Some(Board { grid, width: grid_size, height: grid_size })
        } else {
            None
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> i32 {
        let mut n = 0;

        for nx in vec![x.checked_sub(1), Some(x), x.checked_add(1)] {
            for ny in vec![y.checked_sub(1), Some(y), y.checked_add(1)] {
                match (nx, ny) {
                    (Some(nx), Some(ny)) if nx == x && ny == y => (),
                    (Some(nx), Some(ny))
                        if nx < self.width && ny < self.height &&
                        self[nx][ny] == Cell::Alive => n += 1,
                    (_, _) => (),
                }
            }
        }

        n
    }

    pub fn step(&self) -> Board {
        let mut next_board = self.clone();
        for x in 0..self.width {
            for y in 0..self.height {
                match (self[x][y], self.neighbors(x, y)) {
                    (_, 3)          => next_board[x][y] = Cell::Alive,
                    (Cell::Alive, 2) => next_board[x][y] = Cell::Alive,
                    (_, _)          => next_board[x][y] = Cell::Dead,
                }
            }
        }

        next_board
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            grid: self.grid.iter().map(|grid_line| grid_line.clone()).collect(),
            width: self.width,
            height: self.height,
        }
    }
}

impl ops::Index<usize> for Board {
    type Output = Vec<Cell>;

    fn index(&self, idx: usize) -> &Self::Output {
        &self.grid[idx]
    }
}

impl ops::IndexMut<usize> for Board {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        &mut self.grid[idx]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for grid_line in self.grid.iter() {
            let grid_line_str = grid_line.iter().map(|cell| match cell {
                Cell::Alive => "1".to_string(),
                Cell::Dead  => ".".to_string(),
            }).collect::<Vec<String>>().join(" ");
            
            writeln!(f, "{}", grid_line_str)?
        }
        
        Ok(())
    }
}
