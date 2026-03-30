use crate::pieces::types::{BOARD_SIZE, position::Position};

#[derive(PartialEq, Debug)]
pub struct Offset {
  pub dx: i32,
  pub dy: i32,
}

impl std::ops::Add<Offset> for Position {
  type Output = Option<Position>;

  fn add(self, other: Offset) -> Option<Position> {
    let new_x = self.x() as i32 + other.dx;
    let new_y = self.y() as i32 + other.dy;

    if new_x < 0 || new_y < 0 {
      return None;
    }

    if new_x >= BOARD_SIZE as i32 || new_y >= BOARD_SIZE as i32 {
      return None;
    }

    Position::new(new_x as usize, new_y as usize).ok()
  }
}

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Direction {
  Up,
  Down,
  Left,
  Right,
  UpLeft,
  UpRight,
  DownLeft,
  DownRight,
  KnightUpLeft,
  KnightUpRight,
  KnightDownLeft,
  KnightDownRight,
  KnightLeftUp,
  KnightLeftDown,
  KnightRightUp,
  KnightRightDown,
}

impl Direction {
  pub fn to_offset(&self) -> Offset {
    match self {
      Direction::Up => Offset { dx: -1, dy: 0 },
      Direction::Down => Offset { dx: 1, dy: 0 },
      Direction::Left => Offset { dx: 0, dy: -1 },
      Direction::Right => Offset { dx: 0, dy: 1 },
      Direction::UpLeft => Offset { dx: -1, dy: -1 },
      Direction::UpRight => Offset { dx: -1, dy: 1 },
      Direction::DownLeft => Offset { dx: 1, dy: -1 },
      Direction::DownRight => Offset { dx: 1, dy: 1 },
      Direction::KnightUpLeft => Offset { dx: -2, dy: -1 },
      Direction::KnightUpRight => Offset { dx: -2, dy: 1 },
      Direction::KnightDownLeft => Offset { dx: 2, dy: -1 },
      Direction::KnightDownRight => Offset { dx: 2, dy: 1 },
      Direction::KnightLeftUp => Offset { dx: -1, dy: -2 },
      Direction::KnightLeftDown => Offset { dx: 1, dy: -2 },
      Direction::KnightRightUp => Offset { dx: -1, dy: 2 },
      Direction::KnightRightDown => Offset { dx: 1, dy: 2 },
    }
  }

  pub fn from_offset(offset: Offset) -> Option<Self> {
    match offset {
      Offset { dx: -1, dy: 0 } => Some(Direction::Up),
      Offset { dx: 1, dy: 0 } => Some(Direction::Down),
      Offset { dx: 0, dy: -1 } => Some(Direction::Left),
      Offset { dx: 0, dy: 1 } => Some(Direction::Right),
      Offset { dx: -1, dy: -1 } => Some(Direction::UpLeft),
      Offset { dx: -1, dy: 1 } => Some(Direction::UpRight),
      Offset { dx: 1, dy: -1 } => Some(Direction::DownLeft),
      Offset { dx: 1, dy: 1 } => Some(Direction::DownRight),
      Offset { dx: -2, dy: -1 } => Some(Direction::KnightUpLeft),
      Offset { dx: -2, dy: 1 } => Some(Direction::KnightUpRight),
      Offset { dx: 2, dy: -1 } => Some(Direction::KnightDownLeft),
      Offset { dx: 2, dy: 1 } => Some(Direction::KnightDownRight),
      Offset { dx: -1, dy: -2 } => Some(Direction::KnightLeftUp),
      Offset { dx: 1, dy: -2 } => Some(Direction::KnightLeftDown),
      Offset { dx: -1, dy: 2 } => Some(Direction::KnightRightUp),
      Offset { dx: 1, dy: 2 } => Some(Direction::KnightRightDown),
      _ => None,
    }
  }
}

pub enum MovementPattern {
  AppliableOnce(Vec<Direction>),
  AppliableTwice(Vec<Direction>), // Only for Pawn
  AppliableMultiple(Vec<Direction>),
}

impl MovementPattern {
  pub fn new_appliable_once(directions: Vec<Direction>) -> Self {
    MovementPattern::AppliableOnce(directions)
  }

  pub fn new_appliable_multiple(directions: Vec<Direction>) -> Self {
    MovementPattern::AppliableMultiple(directions)
  }

  pub fn new_appliable_twice(directions: Vec<Direction>) -> Self {
    MovementPattern::AppliableTwice(directions)
  }

  pub fn construct_path(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Option<Vec<Position>> {
    match self {
      MovementPattern::AppliableOnce(move_directions) => {
        let move_direction = Direction::from_offset(Offset {
          dx: target_position.x() as i32 - current_position.x() as i32,
          dy: target_position.y() as i32 - current_position.y() as i32,
        });

        match move_direction {
          Some(direction) => {
            if move_directions.contains(&direction) {
              return Some(vec![target_position]);
            } else {
              return None;
            }
          }
          None => {
            return None;
          }
        }
      }
      MovementPattern::AppliableTwice(moving_directions) => {
        for moving_direction in moving_directions {
          let mut path = vec![];
          let mut current = current_position;

          for _ in 0..2 {
            if let Some(next) = current + moving_direction.to_offset() {
              path.push(next);
              if next == target_position {
                return Some(path);
              }
              current = next;
            } else {
              break;
            }
          }
        }
        None
      }
      MovementPattern::AppliableMultiple(moving_directions) => {
        for moving_direction in moving_directions {
          let mut path = vec![];
          let mut current = current_position;

          while let Some(next) = current + moving_direction.to_offset() {
            path.push(next);
            if next == target_position {
              return Some(path);
            }
            current = next;
          }
        }
        None
      }
    }
  }
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum SpecialMoveValidationAction {
  EnemyPieceExists,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum SpecialMove {
  EnPassant(SpecialMoveValidationAction),
}
