use crate::chessboard::MoveResult;
use crate::game::Game;
use crate::pieces::types::position::Position;
use crate::presenters::Presenter;
use crate::ui::GameUI;
use std::io;

pub struct CmdUI;

impl GameUI for CmdUI {
  fn start_game_loop(&mut self, game: &mut Game) {
    loop {
      game.render();

      let mut input = String::new();
      println!("Enter your move (e.g., e2 e4): ");
      io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

      let positions: Vec<&str> = input.split_whitespace().collect();

      if positions.len() != 2 {
        println!("Invalid input. Please enter two positions.");
        continue;
      }

      let start_pos = Position::from_str(positions[0]);
      let end_pos = Position::from_str(positions[1]);

      if start_pos.is_err() || end_pos.is_err() {
        println!("Invalid position format. Please try again.");
        continue;
      }

      let start_pos = start_pos.unwrap();
      let end_pos = end_pos.unwrap();

      if start_pos == end_pos {
        println!("Start and end positions cannot be the same.");
        continue;
      }

      match game.play(start_pos, end_pos) {
        Ok(res) => {
          println!("Move successful!");
          match res {
            MoveResult::None => (),
            MoveResult::CheckKing => {
              println!("Check! You need to protect your king.");
            }
            MoveResult::CanUpgradePiece => {
              self.handle_upgrade_piece(game, end_pos)
            }
          }
        }
        Err(e) => println!("Error: {}", e),
      }
    }
  }

  fn handle_upgrade_piece(
    &mut self,
    game: &mut Game,
    upgrade_position: Position,
  ) {
    println!("You can upgrade your piece!");
    loop {
      println!(
        "Enter the index of the dead piece you want to upgrade to (e.g., 0, 1, 2): "
      );
      let mut index_input = String::new();
      io::stdin()
        .read_line(&mut index_input)
        .expect("Failed to read line");

      match index_input.trim().parse::<usize>() {
        Ok(index) => {
          if let Err(e) = game.upgrade_piece(index, upgrade_position) {
            println!("Error: {}. Please try again.", e);
            continue;
          }

          println!("Piece upgraded successfully!");
          break;
        }
        Err(_) => {
          println!("Invalid input. Please enter a valid index.");
        }
      }
    }
  }
}
