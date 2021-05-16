use std::io;
use std::io::{Error, ErrorKind};
use crate::engine::{Cell, Board};

/// A simple Parser trait for converting a String representation into a
/// `[Board]`.
pub(crate) trait Parser {
  const MALFORMED_INPUT_ERR: io::Error = Error::new(ErrorKind::InvalidData, "Malformed input file.");
  fn parse_board(&self, str: &str) -> Result<Board, io::Error>;
}

/// A basic parser named after one of my professors, Dr. Massingill. We were
/// assigned a homework to implement Game of Life in C for our Low-Level class,
/// and this is the format of file that we used.
/// 
/// The first line, `n` is a single int that defines the size of the board.
/// What follows is `n` lines of `n` symbols representing live and dead cells,
/// where each cell is separated by a single space. A live cell is represented
/// with the character `1`, while dead cells are represented with the character
/// `.`.
/// 
/// # Example
/// 
/// From `input/game-of-life-in1.txt`:
/// 
/// `
/// 6
/// . . . . . .
/// 1 1 1 . . .
/// . . . . . .
/// . . . . 1 .
/// . . . . 1 .
/// . . . . 1 .
/// `
#[derive(Debug)]
pub(crate) struct MassingillParser {}

impl Parser for MassingillParser {
  fn parse_board(&self, str: &str) -> Result<Board, io::Error> {
    let mut str_iter = str.lines();

    let grid_size_str = str_iter.next()
      .ok_or(Error::new(ErrorKind::UnexpectedEof, "Malformed input file."))?;
    let grid_size = grid_size_str.parse::<usize>()
      .map_err(|e| Error::new(ErrorKind::InvalidData, e))?;
    let grid: Vec<Cell> = str_iter.flat_map(|grid_line|
      grid_line.split(" ").map(|grid_char| match grid_char {
        "1" => Cell::Alive,
        _   => Cell::Dead,
      })
    ).collect();

    if grid_size*grid_size == grid.len() {
      Ok(Board { grid, width: grid_size, height: grid_size })
    } else {
      Err(Self::MALFORMED_INPUT_ERR)
    }
  }
}

#[cfg(test)]
mod test {
  use super::*;

  fn default_mass_parser() -> MassingillParser {
    MassingillParser {}
  }

  #[test]
  fn mass_parser_should_fail_on_empty_string() {
    assert!(default_mass_parser().parse_board("").is_err());
  }

  #[test]
  fn mass_parser_should_fail_on_malformed_string() {
    assert!(default_mass_parser().parse_board("10\n1 . .").is_err());
  }

  #[test]
  fn mass_parser_should_succeed() {
    assert!(default_mass_parser().parse_board("0").is_ok());
    assert!(default_mass_parser().parse_board("1\n.").is_ok());
    assert!(default_mass_parser().parse_board("2\n1 .\n. 1").is_ok());
    let board_str = "\
4
1 . 1 .
. . . .
1 1 1 1
. 1 . 1";
    assert!(default_mass_parser().parse_board(board_str).is_ok());
  }
}