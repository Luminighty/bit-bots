use crate::{game::data::{map::Map, state::GameState, Bot, StepResult}, message::BotAction};

pub fn update_bots(game: &mut GameState) {
	let mut actions = Vec::new();

	for (id, bot) in game.bots.iter_mut() {
		match bot.step() {
			StepResult::Idle => {},
			StepResult::Step => {},
			StepResult::Finished(action) => {
				actions.push((*id, action))
			},
		}
	}

	for (id, action) in actions {
		execute_action(game, &id, action);
	}
}

fn execute_action(game: &mut GameState, id: &usize, action: BotAction) {
	match (game.bots.get_mut(id), action) {
		(Some(bot), BotAction::MoveUp) => try_to_move(&game.map, bot, 0, 1),
		(Some(bot), BotAction::MoveDown) => try_to_move(&game.map, bot, 0, -1),
		(Some(bot), BotAction::MoveLeft) => try_to_move(&game.map, bot, 1, 0),
		(Some(bot), BotAction::MoveRight) => try_to_move(&game.map, bot, -1, 0),
		(None, _) => {}
	}
}

fn try_to_move(map: &Map, bot: &mut Bot, x: i32, y: i32) {
	if !map.is_solid(x, y) {
		bot.x += x;
		bot.y += y;
	}
}
