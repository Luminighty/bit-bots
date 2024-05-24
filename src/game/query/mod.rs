use crate::message::{Query, QueryData, QueryError, QueryResult};
use super::data::state::GameState;

pub fn query(game: &GameState, query: Query) -> QueryResult {
	match query {
		Query::Bot { id } => {
			if let Some(bot) = game.bots.get(&id) {
				Ok(bot.query_data())
			} else {
				Err(QueryError::BotNotFound { id })
			}
		},
		Query::Bots { } => {
			Ok(QueryData::Bots { 
				bots: game.bots.keys().map(|key| *key).collect()
			})
		}
	}
}
