use std::io;
use std::io::{Error, ErrorKind};
use crate::engine::{Cell, Board};

pub(crate) trait Parser {
  fn parse_board(&self, str: &str) -> Result<Board, io::Error>;
}

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