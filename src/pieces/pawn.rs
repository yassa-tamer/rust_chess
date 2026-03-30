use crate::pieces::traits::Movable;
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{
  Direction, MovementPattern, SpecialMove, SpecialMoveValidationAction,
};
use crate::pieces::types::position::Position;

const PAWN_X_START_POSITIONS: [usize; 2] = [
  1, // White
  6, // Black
];

const WHITE_PAWN_UPGRADE_X_POSITION: usize = 7;
const BLACK_PAWN_UPGRADE_X_POSITION: usize = 0;

#[derive(Clone, Copy)]
pub struct Pawn {
  color: Color,
}

impl Pawn {
  pub fn new(color: Color) -> Self {
    Pawn { color }
  }

  fn get_en_passant_movement_pattern(&self) -> MovementPattern {
    let movement_directions = match self.color {
      Color::White => vec![Direction::DownLeft, Direction::DownRight],
      Color::Black => vec![Direction::UpLeft, Direction::UpRight],
    };

    MovementPattern::new_appliable_once(movement_directions)
  }
}

impl Pawn {
  pub fn color(&self) -> &Color {
    &self.color
  }

  pub fn can_upgrade(&self, current_position: Position) -> bool {
    match self.color {
      Color::White => current_position.x() == WHITE_PAWN_UPGRADE_X_POSITION,
      Color::Black => current_position.x() == BLACK_PAWN_UPGRADE_X_POSITION,
    }
  }
}

impl Movable for Pawn {
  fn movement_pattern(&self, current_position: Position) -> MovementPattern {
    let offsets = match self.color {
      Color::White => vec![Direction::Down],
      Color::Black => vec![Direction::Up],
    };

    // Pawns can move two squares forward from their starting position
    if PAWN_X_START_POSITIONS
      .iter()
      .any(|x| *x == current_position.x())
    {
      MovementPattern::new_appliable_twice(offsets)
    } else {
      // Pawns can only move one square forward otherwise
      MovementPattern::new_appliable_once(offsets)
    }
  }

  fn can_reach_via_special_move(
    &self,
    current_position: Position,
    target_position: Position,
  ) -> Result<SpecialMove, ()> {
    let movement_pattern = self.get_en_passant_movement_pattern();

    if let MovementPattern::Once(move_directions) = movement_pattern {
      for move_direction in move_directions {
        let offset = move_direction.to_offset();
        let target_x = current_position.x() as i32 + offset.dx;
        let target_y = current_position.y() as i32 + offset.dy;

        if target_x == target_position.x() as i32
          && target_y == target_position.y() as i32
        {
          return Ok(SpecialMove::EnPassant(
            SpecialMoveValidationAction::EnemyPieceExists,
          ));
        }
      }
    }

    Err(())
  }
}
