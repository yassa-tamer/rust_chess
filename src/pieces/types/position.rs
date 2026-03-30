use crate::pieces::types::BOARD_SIZE;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
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

    let x = chars[0].to_digit(10).unwrap() as usize - 1;
    let y = (chars[1] as u8 - b'A') as usize;

    Position::new(x, y).map_err(|_| "Position out of bounds".to_string())
  }
}
