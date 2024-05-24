pub mod system;
pub mod data;
pub mod query;
pub mod command;

pub use system::update;
pub use query::query;
pub use command::queue_command;

use self::data::{GameState, Bot};

pub fn setup() -> GameState {
	let mut game = GameState::new();

	game.add_bot(Bot::new(0, 10, 10));
	game.add_bot(Bot::new(1, 13, 10));
	game.add_bot(Bot::new(2, 13, 13));
	game.add_bot(Bot::new(3, 10, 13));

	game
}
