use crate::chessboard::{Chessboard, MoveResult};
use crate::errors::{MoveError, UpgradeError};
use crate::pieces::traits::Movable;
use crate::pieces::types::color::Color;
use crate::pieces::types::move_direction::{
  SpecialMove, SpecialMoveValidationAction,
};
use crate::pieces::types::position::Position;
use std::collections::HashMap;

type SpecialMoveValidator =
  Box<dyn Fn(&BoardManager, Position, Position) -> bool>;

pub struct BoardManager {
  chessboard: Chessboard,
}

impl BoardManager {
  pub fn new(chessboard: Chessboard) -> Self {
    BoardManager { chessboard }
  }

  pub fn move_piece(
    &mut self,
    piece_position: Position,
    target_position: Position,
    current_player_color: Color,
  ) -> Result<MoveResult, MoveError> {
    self.can_apply_move(
      piece_position,
      target_position,
      current_player_color,
    )?;

    // apply the move safely
    let res = self
      .chessboard
      .move_piece(piece_position, target_position)?;

    // check if the king is checked after the upgrade
    if res == MoveResult::CanUpgradePiece {
      return Ok(res);
    }

    if self.is_king_checked(current_player_color) {
      return Ok(MoveResult::CheckKing);
    }

    Ok(res)
  }

  fn can_apply_move(
    &self,
    piece_position: Position,
    target_position: Position,
    current_player_color: Color,
  ) -> Result<(), MoveError> {
    self.validate_move_basics(piece_position, current_player_color)?;

    let moving_piece = self.chessboard.get_piece(piece_position).unwrap();

    let special_move_attempt =
      moving_piece.can_reach_via_special_move(piece_position, target_position);

    let can_reach = {
      let can_step =
        self.get_can_step_checker(target_position, current_player_color);
      moving_piece.can_reach(piece_position, target_position, &can_step)
    };

    if !can_reach && special_move_attempt.is_err() {
      return Err(MoveError::InvalidMove);
    }

    #[allow(clippy::collapsible_if)]
    if let Some(special_move_action) =
      self.extract_special_move(special_move_attempt)
    {
      if !self.validate_special_move(
        special_move_action,
        piece_position,
        target_position,
      ) {
        return Err(MoveError::InvalidSpecialMove);
      }
    }

    Ok(())
  }

  fn validate_move_basics(
    &self,
    piece_position: Position,
    current_player_color: Color,
  ) -> Result<(), MoveError> {
    self.validate_piece_exists(piece_position)?;
    self.validate_player_owns_piece(piece_position, current_player_color)?;

    Ok(())
  }

  fn get_can_step_checker(
    &self,
    target_position: Position,
    current_player_color: Color,
  ) -> impl Fn(Position) -> bool + '_ {
    move |pos| {
      if pos == target_position {
        match self.chessboard.get_piece(pos) {
          Some(piece) => !piece.is_of_color(current_player_color),
          None => true,
        }
      } else {
        self.chessboard.is_position_empty(pos)
      }
    }
  }

  fn extract_special_move(
    &self,
    result: Result<SpecialMove, ()>,
  ) -> Option<SpecialMoveValidationAction> {
    match result {
      Ok(SpecialMove::EnPassant(action)) => Some(action),
      Err(_) => None,
    }
  }

  fn validate_special_move(
    &self,
    action: SpecialMoveValidationAction,
    from: Position,
    to: Position,
  ) -> bool {
    let validation_fn = self.get_special_move_validation_action(action);
    validation_fn(self, from, to)
  }

  fn validate_piece_exists(
    &self,
    piece_position: Position,
  ) -> Result<(), MoveError> {
    if self.chessboard.is_position_empty(piece_position) {
      return Err(MoveError::NoPieceAtPosition);
    }

    Ok(())
  }

  fn validate_player_owns_piece(
    &self,
    piece_position: Position,
    current_player_color: Color,
  ) -> Result<(), MoveError> {
    if !self.can_player_move_piece_at(piece_position, current_player_color) {
      return Err(MoveError::NotYourPiece);
    }

    Ok(())
  }

  fn is_king_checked(&self, current_player_color: Color) -> bool {
    let enemy_color = current_player_color.next();
    let Some(king_position) =
      self.chessboard.get_king_position(enemy_color)
    else {
      return false;
    };

    for position in self.chessboard.get_all_positions() {
      if self.chessboard.is_position_empty(position) {
        continue;
      }
      let piece = self.chessboard.get_piece(position).unwrap();

      if piece.is_of_color(current_player_color)
        && self
          .can_apply_move(position, king_position, current_player_color)
          .is_ok()
      {
        return true;
      }
    }
    false
  }

  pub fn upgrade_piece(
    &mut self,
    piece_index_in_dead_pieces_vector: usize,
    current_player_color: Color,
    target_position: Position,
  ) -> Result<MoveResult, UpgradeError> {
    self.chessboard.upgrade_piece(
      piece_index_in_dead_pieces_vector,
      current_player_color,
      target_position,
    )?;

    if self.is_king_checked(current_player_color) {
      return Ok(MoveResult::CheckKing);
    }

    Ok(MoveResult::None)
  }

  fn can_player_move_piece_at(
    &self,
    position: Position,
    player_color: Color,
  ) -> bool {
    self
      .chessboard
      .get_piece(position)
      .is_some_and(|piece| piece.is_of_color(player_color))
  }

  fn get_special_move_validation_action(
    &self,
    special_move_validation: SpecialMoveValidationAction,
  ) -> SpecialMoveValidator {
    let mut special_move_validation_functions = HashMap::new();
    special_move_validation_functions.insert(
      SpecialMoveValidationAction::EnemyPieceExists,
      |board_manager: &BoardManager,
       piece_position: Position,
       target_position: Position| {
        if board_manager.chessboard.is_position_empty(target_position) {
          return false;
        }
        #[allow(clippy::collapsible_if)]
        if let Some(piece) = board_manager.chessboard.get_piece(target_position)
        {
          if piece.is_of_color(
            *board_manager
              .chessboard()
              .get_piece(piece_position)
              .unwrap()
              .color(),
          ) {
            return false;
          }
        }
        true
      },
    );

    Box::new(
      special_move_validation_functions
        .remove(&special_move_validation)
        .unwrap(),
    )
  }

  pub fn chessboard(&self) -> &Chessboard {
    &self.chessboard
  }
}
