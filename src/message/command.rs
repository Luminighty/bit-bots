use crate::game::data::bot::BotId;

#[derive(Debug, Clone, Copy)]
pub enum Command {
	Bot { id: BotId, action: BotAction }
}

#[derive(Debug)]
pub enum CommandResult {
	Ack,
	BotNotFound { id: BotId }
}

#[derive(Clone, Copy, Debug)]
pub enum BotAction {
	MoveUp, MoveDown, MoveLeft, MoveRight
}


impl BotAction {
	pub fn time(&self) -> usize {
		match self {
			BotAction::MoveDown |
			BotAction::MoveLeft |
			BotAction::MoveRight |
			BotAction::MoveUp => 10,
		}
	}
}
