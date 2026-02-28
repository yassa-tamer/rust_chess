#[cfg(test)]
mod tests {
  use crate::board_manager::BoardManager;
  use crate::chessboard::MoveResult;
  use crate::chessboard::{Chessboard, ChessboardType};
  use crate::pieces::piece::Piece;
  use crate::pieces::types::{color::Color, position::Position};
  use crate::pieces::{Pawn, Queen};
  use std::array::from_fn;

  #[test]
  fn test_move_piece_valid_move() {
    let board = Chessboard::standard(); // Creating a standard chessboard
    let mut board_manager = BoardManager::new(board);

    // White pawn moves from A2 to A3
    let start_pos = Position::new(1, 0).unwrap(); // A2
    let target_pos = Position::new(2, 0).unwrap(); // A3

    assert!(
      board_manager
        .move_piece(start_pos, target_pos, Color::White)
        .is_ok()
    );
    assert!(board_manager.chessboard().board()[2][0].is_some()); // The pawn should now be at A3
    assert!(board_manager.chessboard().board()[1][0].is_none()); // The original position should be empty
  }

  #[test]
  fn test_move_piece_invalid_move() {
    let board = Chessboard::standard(); // Creating a standard chessboard
    let mut board_manager = BoardManager::new(board);

    // Try to move a black pawn from A7 to A6 when it's white's turn
    let start_pos = Position::new(6, 0).unwrap(); // A7
    let target_pos = Position::new(5, 0).unwrap(); // A6

    let result = board_manager.move_piece(start_pos, target_pos, Color::White);
    assert!(result.is_err()); // Should fail, as it's white's turn
  }

  #[test]
  fn test_capture_piece() {
    let board = Chessboard::standard(); // Creating a standard chessboard
    let mut board_manager = BoardManager::new(board);

    // Move a pawn in front of the bishop to free it to move
    board_manager
      .move_piece(
        Position::new(1, 3).unwrap(),
        Position::new(2, 3).unwrap(),
        Color::White,
      )
      .unwrap();

    // Move the black pawn from (6, 7) to (5, 7)
    board_manager
      .move_piece(
        Position::new(6, 7).unwrap(),
        Position::new(5, 7).unwrap(),
        Color::Black,
      )
      .unwrap();

    // Move the white piece from (0, 2) to (5, 7)
    // Assuming the piece at (0, 2) is a bishop (or another piece that can move like this)
    board_manager
      .move_piece(
        Position::new(0, 2).unwrap(),
        Position::new(5, 7).unwrap(),
        Color::White,
      )
      .unwrap();

    // The bishop captures the pawn, so it should now be in the dead pieces list
    assert_eq!(board_manager.chessboard().black_dead_pieces().len(), 1);
  }

  #[test]
  fn test_en_passant_capture() {
    // Custom board setup
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    custom_board[4][4] = Some(Piece::Pawn(Pawn::new(Color::White)));
    custom_board[5][3] = Some(Piece::Pawn(Pawn::new(Color::Black)));

    let mut board_manager =
      BoardManager::new(Chessboard::new(custom_board, Vec::new(), Vec::new()));

    let result = board_manager.move_piece(
      Position::new(4, 4).unwrap(),
      Position::new(5, 3).unwrap(),
      Color::White,
    );

    assert!(result.is_ok());
    assert!(board_manager.chessboard().board()[5][3].is_some()); // White pawn should be on d6
    assert_eq!(board_manager.chessboard().black_dead_pieces().len(), 1); // Confirm capture
  }

  #[test]
  fn test_pawn_upgrade_triggers_upgrade_result() {
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));
    custom_board[6][0] = Some(Piece::Pawn(Pawn::new(Color::White)));

    let mut board_manager =
      BoardManager::new(Chessboard::new(custom_board, Vec::new(), Vec::new()));

    let result = board_manager.move_piece(
      Position::new(6, 0).unwrap(),
      Position::new(7, 0).unwrap(),
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::CanUpgradePiece)));
  }

  #[test]
  fn test_upgrade_piece_replaces_board_piece() {
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    // Set up white pawn at 6, 0 (A7) and black pawn at 7, 1 (B8)
    custom_board[6][0] = Some(Piece::Pawn(Pawn::new(Color::White)));
    custom_board[7][1] = Some(Piece::Queen(Queen::new(Color::Black)));

    let mut board_manager =
      BoardManager::new(Chessboard::new(custom_board, Vec::new(), Vec::new()));

    // Move white pawn from A7 to B8, capturing the black piece and triggering upgrade
    let result = board_manager.move_piece(
      Position::new(6, 0).unwrap(),
      Position::new(7, 1).unwrap(),
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::CanUpgradePiece)));
    assert_eq!(board_manager.chessboard().black_dead_pieces().len(), 1);

    // Perform upgrade with the captured black piece
    board_manager
      .upgrade_piece(0, Color::Black, Position::new(7, 1).unwrap())
      .expect("Failed to upgrade piece");

    // Confirm the upgrade replaced the pawn on the board
    assert!(board_manager.chessboard().board()[7][1].is_some());
    let piece = board_manager.chessboard().board()[7][1].as_ref().unwrap();
    assert!(!matches!(piece, Piece::Pawn(_))); // make sure it's not a pawn anymore
  }

  #[test]
  fn test_king_check_after_move() {
    use crate::pieces::{King, Rook};

    // Create a custom board where white rook checks black king
    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    // Place black king at E8 (row 7, col 4)
    custom_board[7][4] = Some(Piece::King(King::new(Color::Black)));

    // Place white rook at E1 (row 0, col 4)
    custom_board[0][4] = Some(Piece::Rook(Rook::new(Color::White)));

    let mut board_manager =
      BoardManager::new(Chessboard::new(custom_board, Vec::new(), Vec::new()));

    // Move rook from E1 to E7 (just before the king), to put the king in check
    let result = board_manager.move_piece(
      Position::new(0, 4).unwrap(), // E1
      Position::new(6, 4).unwrap(), // E7
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::CheckKing))); // Should result in check
  }

  #[test]
  fn test_king_not_checked_after_safe_move() {
    use crate::pieces::{King, Rook};

    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    custom_board[7][4] = Some(Piece::King(King::new(Color::Black))); // Black king at E8
    custom_board[0][0] = Some(Piece::Rook(Rook::new(Color::White))); // White rook at A1

    let mut board_manager =
      BoardManager::new(Chessboard::new(custom_board, Vec::new(), Vec::new()));

    // Move rook to A2, not affecting the king
    let result = board_manager.move_piece(
      Position::new(0, 0).unwrap(),
      Position::new(1, 0).unwrap(),
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::None))); // No check triggered
  }

  #[test]
  fn test_king_check_after_upgrade() {
    use crate::pieces::{King, Pawn, Queen};

    let mut custom_board: ChessboardType = from_fn(|_| from_fn(|_| None));

    // White pawn at B7 (6,1), black king at G8 (7,5)
    custom_board[6][1] = Some(Piece::Pawn(Pawn::new(Color::White)));
    custom_board[7][5] = Some(Piece::King(King::new(Color::Black)));

    // Add a Queen to the white dead pieces (to be used for promotion)
    let white_dead_pieces: Vec<Piece> =
      vec![Piece::Queen(Queen::new(Color::White))];

    let mut board_manager = BoardManager::new(Chessboard::new(
      custom_board,
      white_dead_pieces,
      Vec::new(), // No black dead pieces
    ));

    // Move pawn from B7 to B8 (no capture), triggers upgrade
    let result = board_manager.move_piece(
      Position::new(6, 1).unwrap(), // B7
      Position::new(7, 1).unwrap(), // B8
      Color::White,
    );

    assert!(matches!(result, Ok(MoveResult::CanUpgradePiece)));
    assert_eq!(board_manager.chessboard().white_dead_pieces().len(), 1);

    // Upgrade the pawn to a Queen (from white's dead pieces)
    let upgrade_result = board_manager.upgrade_piece(
      0,
      Color::White,
      Position::new(7, 1).unwrap(), // B8
    );

    // The Queen at B8 should now check the black king at G8
    assert!(matches!(upgrade_result, Ok(MoveResult::CheckKing)));
  }
}
