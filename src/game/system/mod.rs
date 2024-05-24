use super::data::state::GameState;

mod bot;

pub fn update(game: &mut GameState) {
	bot::update_bots(game);

}