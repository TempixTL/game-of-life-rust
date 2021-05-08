use std::fmt;
use std::fmt::Formatter;
use std::io::Error;
use std::ops;
use crate::cfg::Config;
use crate::parser::Parser;

/// Runs the Game of Life according to the given [Config]. Prints results to
/// `stdout`.
/// 
/// # Arguments
/// * `config` - The configuration specifying how to run the Game of Life
/// simulation.
pub fn run(config: Config) -> Result<(), Error> {
    let Config { starting_board, steps, parser } = config;
    let mut current_board = Board::parse(&starting_board, &*parser)?;
    for iteration in 0..=steps {
        if iteration == 0 {
            println!("Initial board:");
        } else {
            println!("Board after step {}:", iteration)
        }
        println!("{}", current_board);
        current_board = current_board.step();
    }

    Ok(())
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Cell {
    Alive,
    Dead,
}

/// A Board which stores a grid of [`Cell`] in row-major order.
#[derive(Debug)]
pub(crate) struct Board {
    pub grid: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl Board {
    pub fn parse(str: &str, parser: &dyn Parser) -> Result<Board, Error> {
        parser.parse_board(str)
    }

    pub fn neighbors(&self, r: usize, c: usize) -> i32 {
        let mut n = 0;

        for nr in vec![r.checked_sub(1), Some(r), r.checked_add(1)] {
            for nc in vec![c.checked_sub(1), Some(c), c.checked_add(1)] {
                match (nr, nc) {
                    (Some(nr), Some(nc))
                        if !(nr == r && nc == c) &&
                        nc < self.width && nr < self.height &&
                        self[(nr, nc)] == Cell::Alive => n += 1,
                    (_, _) => (),
                }
            }
        }

        n
    }

    pub fn step(&self) -> Board {
        let mut next_board = self.clone();
        for c in 0..self.width {
            for r in 0..self.height {
                match (self[(r, c)], self.neighbors(r, c)) {
                    (_, 3)           => next_board[(r, c)] = Cell::Alive,
                    (Cell::Alive, 2) => next_board[(r, c)] = Cell::Alive,
                    (_, _)           => next_board[(r, c)] = Cell::Dead,
                }
            }
        }

        next_board
    }
}

impl Clone for Board {
    fn clone(&self) -> Board {
        Board {
            grid: self.grid.clone(),
            width: self.width,
            height: self.height,
        }
    }
}

impl ops::Index<(usize, usize)> for Board {
    type Output = Cell;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        &self.grid[r*self.width + c]
    }
}

impl ops::IndexMut<(usize, usize)> for Board {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        &mut self.grid[r*self.width + c]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, cell) in self.grid.iter().enumerate() {
            let grid_char = match cell {
                Cell::Alive => "1",
                Cell::Dead  => ".",
            };

            write!(f, "{}", grid_char)?;
            if (i+1) % self.width == 0 {
                writeln!(f)?;
            } else {
                write!(f, " ")?;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::parser::MassingillParser;

    #[test]
    fn board_new_should_return_correct_board() {
        let board_str = "\
2
1 .
. 1";
        match Board::parse(board_str, &MassingillParser {}) {
            Ok(board) => {
                assert_eq!(board[(0,0)], Cell::Alive);
                assert_eq!(board[(1,0)], Cell::Dead);
                assert_eq!(board[(0,1)], Cell::Dead);
                assert_eq!(board[(1,1)], Cell::Alive);
                assert_eq!(board.width, 2);
                assert_eq!(board.height, 2);
            },
            Err(_) => panic!(),
        }
    }

    fn create_test_board() -> Board {
        let board_str = "\
4
1 . 1 .
. . . .
1 1 1 1
. 1 . 1";
        Board::parse(board_str, &MassingillParser {}).unwrap()
    }

    #[test]
    fn board_neighbors_should_correctly_count_neighbors() {
        let board = create_test_board();

        assert_eq!(board.neighbors(0, 0), 0);
        assert_eq!(board.neighbors(1, 1), 5);
        assert_eq!(board.neighbors(2, 2), 4);
        assert_eq!(board.neighbors(2, 1), 3);
        assert_eq!(board.neighbors(3, 3), 2);
        assert_eq!(board.neighbors(0, 3), 1);
    }

    #[test]
    fn board_next_should_correctly_compute_next_board() {
        let board = create_test_board();
        let next_board_str = "\
4
. . . .
1 . . 1
1 1 . 1
1 1 . 1";
        let next_board = Board::parse(next_board_str, &MassingillParser {}).unwrap();
        let next_board_calc = board.step();

        assert_eq!(next_board.width, next_board_calc.width);
        assert_eq!(next_board.height, next_board_calc.height);
        for x in 0..next_board.width {
            for y in 0..next_board.height {
                assert_eq!(next_board[(y,x)], next_board_calc[(y,x)], "Index ({}, {}) of computed board didn't match.", y, x);
            }
        }
    }
}