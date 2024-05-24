use message::Query;

use crate::message::{BotAction, Command};

mod game;
mod message;

fn main() {
  let mut state = game::setup();

  for _ in 0..7 {
    println!(
      "{:?}", 
      game::queue_command(&mut state, Command::Bot { id: 1, action: BotAction::MoveUp })
    );
  }

  for _ in 0..20 {
    game::update(&mut state);
  }

  println!("{:?}", game::query(&state, Query::Bots { }));
  println!("{:?}", game::query(&state, Query::Bot { id: 1 }));
}
