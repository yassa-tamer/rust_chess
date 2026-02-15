use crate::errors::{MoveError, UpgradeError};
use crate::pieces::piece::Piece;
use crate::pieces::types::BOARD_SIZE;
use crate::pieces::types::color::Color;
use crate::pieces::types::position::Position;
use crate::pieces::{Bishop, King, Knight, Pawn, Queen, Rook};
use std::array::from_fn;

pub type ChessboardType = [[Option<Piece>; BOARD_SIZE]; BOARD_SIZE];

#[derive(PartialEq, Debug)]
pub enum MoveResult {
  None,
  CanUpgradePiece,
  CheckKing,
}

const FIRST_WHITE_ROW_X_POS: usize = 0;
const WHITE_PAWNS_ROW_X_POS: usize = 1;

const FIRST_BLACK_ROW_X_POS: usize = 7;
const BLACK_PAWNS_ROW_X_POS: usize = 6;

pub struct Chessboard {
  chessboard: ChessboardType,
  white_dead_pieces: Vec<Piece>,
  black_dead_pieces: Vec<Piece>,
}

impl Chessboard {
  pub fn new(
    chessboard: ChessboardType,
    white_dead_pieces: Vec<Piece>,
    black_dead_pieces: Vec<Piece>,
  ) -> Self {
    Chessboard {
      chessboard,
      white_dead_pieces,
      black_dead_pieces,
    }
  }

  pub fn empty() -> Self {
    Self::new(from_fn(|_| from_fn(|_| None)), Vec::new(), Vec::new())
  }

  pub fn standard() -> Self {
    let mut board = Chessboard::empty();

    board.initialize_pieces(
      Color::White,
      FIRST_WHITE_ROW_X_POS,
      WHITE_PAWNS_ROW_X_POS,
    );
    board.initialize_pieces(
      Color::Black,
      FIRST_BLACK_ROW_X_POS,
      BLACK_PAWNS_ROW_X_POS,
    );

    board
  }

  fn initialize_pieces(
    &mut self,
    color: Color,
    first_row: usize,
    pawns_row: usize,
  ) {
    let back_row: [Piece; 8] = [
      Piece::Rook(Rook::new(color)),
      Piece::Knight(Knight::new(color)),
      Piece::Bishop(Bishop::new(color)),
      Piece::Queen(Queen::new(color)),
      Piece::King(King::new(color)),
      Piece::Bishop(Bishop::new(color)),
      Piece::Knight(Knight::new(color)),
      Piece::Rook(Rook::new(color)),
    ];

    for (col, piece) in back_row.into_iter().enumerate() {
      self.set_piece(Position::new(first_row, col).unwrap(), Some(piece));
    }

    for col in 0..BOARD_SIZE {
      self.set_piece(
        Position::new(pawns_row, col).unwrap(),
        Some(Piece::Pawn(Pawn::new(color))),
      );
    }
  }

  pub fn get_piece(&self, pos: Position) -> Option<&Piece> {
    self.chessboard[pos.x()][pos.y()].as_ref()
  }

  fn take_piece(&mut self, pos: Position) -> Option<Piece> {
    self.chessboard[pos.x()][pos.y()].take()
  }

  fn set_piece(&mut self, pos: Position, piece: Option<Piece>) {
    self.chessboard[pos.x()][pos.y()] = piece;
  }

  pub fn is_position_empty(&self, position: Position) -> bool {
    self.chessboard[position.x()][position.y()].is_none()
  }

  pub fn board(&self) -> &ChessboardType {
    &self.chessboard
  }

  pub fn white_dead_pieces(&self) -> &Vec<Piece> {
    &self.white_dead_pieces
  }

  pub fn black_dead_pieces(&self) -> &Vec<Piece> {
    &self.black_dead_pieces
  }

  pub fn capture_piece(&mut self, target_position: Position) {
    if let Some(target_piece) = self.take_piece(target_position) {
      if *target_piece.color() == Color::White {
        self.white_dead_pieces.push(target_piece);
      } else {
        self.black_dead_pieces.push(target_piece);
      }
    }
  }

  pub fn move_piece(
    &mut self,
    piece_position: Position,
    target_position: Position,
  ) -> Result<MoveResult, MoveError> {
    let piece = self
      .take_piece(piece_position)
      .ok_or(MoveError::NoPieceAtPosition)?;

    self.capture_piece(target_position);

    self.set_piece(target_position, Some(piece));
    self.set_piece(piece_position, None);

    let piece = self.get_piece(target_position).unwrap();

    let can_upgrade = match piece {
      Piece::Pawn(pawn) => pawn.can_upgrade(target_position),
      _ => false,
    };

    if can_upgrade {
      return Ok(MoveResult::CanUpgradePiece);
    }

    Ok(MoveResult::None)
  }

  pub fn upgrade_piece(
    &mut self,
    piece_index_in_dead_pieces_vector: usize,
    current_player_color: Color,
    target_position: Position,
  ) -> Result<(), UpgradeError> {
    let dead_pieces = match current_player_color {
      Color::White => &mut self.white_dead_pieces,
      Color::Black => &mut self.black_dead_pieces,
    };

    if piece_index_in_dead_pieces_vector >= dead_pieces.len() {
      return Err(UpgradeError::InvalidPieceIndex);
    }

    let piece_to_upgrade =
      dead_pieces.remove(piece_index_in_dead_pieces_vector);

    self.set_piece(target_position, Some(piece_to_upgrade));

    Ok(())
  }

  pub fn get_king_position(&self, color: Color) -> Option<Position> {
    for position in self.get_all_positions() {
      if let Some(piece) = self.get_piece(position) {
        if matches!(piece, Piece::King(_)) && piece.color() == &color {
          return Some(position);
        }
      }
    }
    None
  }

  pub fn get_all_positions(&self) -> Vec<Position> {
    let mut positions = Vec::new();
    for x in 0..BOARD_SIZE {
      for y in 0..BOARD_SIZE {
        positions.push(Position::new(x, y).unwrap());
      }
    }
    positions
  }
}
