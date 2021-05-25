use std::io;
use std::io::{Error, ErrorKind};
use regex::Regex;
use crate::engine::{Cell, Board};

/// A simple Parser trait for converting a String representation into a
/// `[Board]`.
pub(crate) trait Parser {
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
      Err(Error::new(ErrorKind::InvalidData, "Malformed input file."))
    }
  }
}

pub(crate) struct RLEParser {}

impl Parser for RLEParser {
  fn parse_board(&self, str: &str) -> Result<Board, io::Error> {
    let invalid_data_err = |err| Error::new(ErrorKind::InvalidData, err);

    // validate input string
    let re = Regex
      ::new(r"^(?:#.*\n)*x *= *(\d+), *y *= *(\d+).*\n((?:(?:\s*\d*(?:b|o)\s*)*(?:\$|!.*))*)")
      .unwrap();
    let cap = re.captures(str).ok_or(invalid_data_err("Invalid file format."))?;

    let cols = cap.get(1)
      .and_then(|x| x.as_str().parse::<usize>().ok())
      .ok_or(invalid_data_err("Invalid number of columns."))?;
    let rows = cap.get(2)
      .and_then(|x| x.as_str().parse::<usize>().ok())
      .ok_or(invalid_data_err("Invalid number of rows."))?;
    let board_str = cap.get(3).map(|mtch| mtch.as_str())
      .ok_or(invalid_data_err("Invalid board representation."))?
      .trim();

    let board_rows_str = board_str
      .split_terminator(|c| c == '$' || c == '!')
      .collect::<Vec<&str>>();
    if board_rows_str.len() != rows {
      return Err(invalid_data_err(&format!(
        "Number of rows in header does not match. Found {}, header stated {}",
        board_rows_str.len(),
        rows
      )));
    }

    // parse grid string
    let mut grid = Vec::new();
    let cell_re = Regex::new(r"(\d*)(b|o)").unwrap();
    for row_str in board_rows_str {
      let mut col = 0;
      for cap in cell_re.captures_iter(row_str) {
        let cnt = if cap[1].is_empty() { 1 } else {
          cap[1]
            .parse::<usize>()
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))?
        };
        let cell = match &cap[2] {
          "o" => Ok(Cell::Alive),
          "b" => Ok(Cell::Dead),
          _   => Err(invalid_data_err("Invalid cell representation.")),
        }?;

        col += cnt;
        for _ in 0..cnt {
          grid.push(cell);
        }
      }

      while col < cols {
        grid.push(Cell::Dead);
        col += 1;
      }
    }

    if grid.len() == rows*cols {
      Ok(Board { width: cols, height: rows, grid })
    } else {
      Err(invalid_data_err(&format!(
        "Board size (x*y) in header does not match. Found {}, header stated {}",
        grid.len(),
        rows*cols
      )))
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

  fn default_rle_parser() -> RLEParser {
    RLEParser {}
  }

  #[test]
  fn rle_parser_should_succeed() -> Result<(), io::Error> {
    let board_str = "\
#N Gosper glider gun
#C This was the first gun discovered.
#C As its name suggests, it was discovered by Bill Gosper.
x = 36, y = 9, rule = B3/S23
24bo$22bobo$12b2o6b2o12b2o$11bo3bo4b2o12b2o$2o8bo5bo3b2o$2o8bo3bob2o4b
obo$10bo5bo7bo$11bo3bo$12b2o!";
    default_rle_parser().parse_board(board_str)?;
    Ok(())
  }
}
