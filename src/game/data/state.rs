use std::collections::HashMap;
use super::{bot::*, map::{generate_map, Map}};

pub struct GameState {
	pub bots: HashMap<BotId, Bot>,
	pub map: Map,
}


impl GameState {
	pub fn new() -> Self {
		Self {
			bots: HashMap::new() ,
			map: generate_map()
		}
	}

	pub fn add_bot(&mut self, bot: Bot) {
		self.bots.insert(bot.id, bot);
	}
}
