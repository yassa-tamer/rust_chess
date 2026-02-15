use crate::board_manager::BoardManager;
use crate::chessboard::{Chessboard, MoveResult};
use crate::errors::{MoveError, UpgradeError};
use crate::pieces::types::color::Color;
use crate::pieces::types::position::Position;

pub struct Game {
  player_color: Color,
  board_manager: BoardManager,
}

impl Game {
  pub fn new(player_color: Color) -> Self {
    let board = Chessboard::standard();
    let board_manager = BoardManager::new(board);
    Game {
      player_color,
      board_manager,
    }
  }

  pub fn play(
    &mut self,
    piece_position: Position,
    target_position: Position,
  ) -> Result<MoveResult, MoveError> {
    match self.board_manager.move_piece(
      piece_position,
      target_position,
      self.player_color,
    ) {
      Ok(res) => {
        self.player_color = self.player_color.next();
        Ok(res)
      }
      Err(e) => Err(e),
    }
  }

  pub fn upgrade_piece(
    &mut self,
    piece_index: usize,
    upgrade_position: Position,
  ) -> Result<MoveResult, UpgradeError> {
    self
      .board_manager
      // The current player color is the opponent's color because it's changed after a valid move
      .upgrade_piece(piece_index, self.player_color().next(), upgrade_position)
  }

  pub fn board_manager(&self) -> &BoardManager {
    &self.board_manager
  }

  pub fn player_color(&self) -> Color {
    self.player_color
  }
}
