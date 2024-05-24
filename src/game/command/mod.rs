use crate::message::{Command, CommandResult};

use super::data::state::GameState;

pub fn queue_command(game: &mut GameState, command: Command) -> CommandResult {
	match command {
		Command::Bot { id, action } => {
			if let Some(bot) = game.bots.get_mut(&id) {
				bot.queue_command(action);
				CommandResult::Ack
			} else {
				CommandResult::BotNotFound { id }
			}
		}
	}
}
