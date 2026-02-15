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
  pub fn new(x: usize, y: usize) -> Result<Self, ()> {
    if x >= BOARD_SIZE || y >= BOARD_SIZE {
      return Err(());
    }
    Ok(Position { x, y })
  }

  pub fn x(&self) -> usize {
    self.x
  }

  pub fn y(&self) -> usize {
    self.y
  }

  pub fn from_str(position: &str) -> Result<Self, String> {
    if position.len() != 2 {
      return Err("Invalid position format".to_string());
    }
    let chars: Vec<char> = position.chars().collect();

    let x = Self::parse_row(chars[0])?;
    let y = Self::parse_col(chars[1])?;

    Position::new(x, y).map_err(|_| "Position out of bounds".to_string())
  }

  fn parse_row(c: char) -> Result<usize, String> {
    let digit = c
      .to_digit(10)
      .ok_or("Invalid position format".to_string())?;
    if digit < 1 {
      return Err("Invalid position format".to_string());
    }
    Ok(digit as usize - 1)
  }

  fn parse_col(c: char) -> Result<usize, String> {
    if !c.is_ascii_uppercase() || c > 'H' {
      return Err("Invalid position format".to_string());
    }
    Ok((c as u8 - b'A') as usize)
  }
}
