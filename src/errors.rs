use std::fmt;

#[derive(Debug)]
pub enum PositionError {
  InvalidFormat,
  OutOfBounds,
}

impl fmt::Display for PositionError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      PositionError::InvalidFormat => {
        write!(f, "Invalid position format")
      }
      PositionError::OutOfBounds => {
        write!(f, "Position out of bounds")
      }
    }
  }
}

#[derive(Debug)]
pub enum MoveError {
  NoPieceAtPosition,
  NotYourPiece,
  InvalidMove,
  InvalidSpecialMove,
}

impl fmt::Display for MoveError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      MoveError::NoPieceAtPosition => {
        write!(f, "No piece at the given position")
      }
      MoveError::NotYourPiece => write!(f, "Not your piece"),
      MoveError::InvalidMove => write!(f, "Invalid move"),
      MoveError::InvalidSpecialMove => {
        write!(f, "Invalid special move")
      }
    }
  }
}

#[derive(Debug)]
pub enum UpgradeError {
  InvalidPieceIndex,
}

impl fmt::Display for UpgradeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      UpgradeError::InvalidPieceIndex => {
        write!(f, "Invalid index for dead pieces vector")
      }
    }
  }
}
