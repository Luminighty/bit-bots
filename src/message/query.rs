use crate::game::data::BotId;

pub enum Query {
	Bots {},
	Bot { id: BotId }
}

pub type QueryResult = Result<QueryData, QueryError>;

#[derive(Debug)]
pub enum QueryData {
	Bots { bots: Vec<BotId> },
	BotData { x: i32, y: i32, id: BotId }
}

#[derive(Debug)]
pub enum QueryError {
	UnknownQuery,
	BotNotFound { id: BotId }
}