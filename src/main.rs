mod board_manager;
mod chessboard;
mod errors;
mod game;
mod pieces;
mod presenters;
mod tests;
mod ui;

use game::Game;
use pieces::types::color::Color;
use ui::{GameUI, cmd::CmdUI};

fn main() {
  let mut game = Game::new(Color::White);

  // For now, always use CLI
  let mut ui = CmdUI;
  ui.start_game_loop(&mut game);
}
