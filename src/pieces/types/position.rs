use crate::errors::PositionError;
use crate::pieces::types::BOARD_SIZE;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Position {
  x: usize,
  y: usize,
}

impl PartialOrd for Position {
  fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
    Some(self.cmp(other))
  }
}

impl Ord for Position {
  fn cmp(&self, other: &Self) -> std::cmp::Ordering {
    (self.x, self.y).cmp(&(other.x, other.y))
  }
}

impl Position {
  pub fn new(x: usize, y: usize) -> Result<Self, PositionError> {
    if x >= BOARD_SIZE || y >= BOARD_SIZE {
      return Err(PositionError::OutOfBounds);
    }
    Ok(Position { x, y })
  }

  pub fn x(&self) -> usize {
    self.x
  }

  pub fn y(&self) -> usize {
    self.y
  }

  pub fn from_str(position: &str) -> Result<Self, PositionError> {
    if position.len() != 2 {
      return Err(PositionError::InvalidFormat);
    }
    let chars: Vec<char> = position.chars().collect();

    let x = Self::parse_row(chars[0])?;
    let y = Self::parse_col(chars[1])?;

    Position::new(x, y)
  }

  fn parse_row(c: char) -> Result<usize, PositionError> {
    let digit = c.to_digit(10).ok_or(PositionError::InvalidFormat)?;
    if digit < 1 || digit > BOARD_SIZE as u32 {
      return Err(PositionError::InvalidFormat);
    }
    Ok(digit as usize - 1)
  }

  fn parse_col(c: char) -> Result<usize, PositionError> {
    if !c.is_ascii_uppercase() || (c as u8) >= b'A' + BOARD_SIZE as u8 {
      return Err(PositionError::InvalidFormat);
    }
    Ok((c as u8 - b'A') as usize)
  }
}
