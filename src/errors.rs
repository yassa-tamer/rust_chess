use std::error::Error;
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum PositionError {
  InvalidFormat,
  OutOfBounds,
}

impl Error for PositionError {}

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

#[derive(Debug, PartialEq)]
pub enum MoveError {
  NoPieceAtPosition,
  NotYourPiece,
  InvalidMove,
  InvalidSpecialMove,
}

impl Error for MoveError {}

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

#[derive(Debug, PartialEq)]
pub enum UpgradeError {
  InvalidPieceIndex,
}

impl Error for UpgradeError {}

impl fmt::Display for UpgradeError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      UpgradeError::InvalidPieceIndex => {
        write!(f, "Invalid index for dead pieces vector")
      }
    }
  }
}
