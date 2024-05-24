use std::collections::VecDeque;

use crate::message::{BotAction, QueryData};

pub type BotId = usize;

pub enum StepResult {
	Idle,
	Finished(BotAction),
	Step,
}

pub struct Bot {
	pub id: BotId,
	pub x: i32,
	pub y: i32,
	pub delay: usize,
	queue: VecDeque<BotAction>
}

impl Bot {
	pub fn new(id: BotId, x: i32, y: i32) -> Self {
		Self {
			id, x, y,
			delay: 0,
			queue: VecDeque::new()
		}
	}

	pub fn query_data(&self) -> QueryData {
		QueryData::BotData { 
			x: self.x, 
			y: self.y, 
			id: self.id
		}
	}

	pub fn queue_command(&mut self, command: BotAction) {
		if self.queue.is_empty() {
			self.delay = command.time();
		}
		self.queue.push_back(command)
	}

	pub fn clear_commands(&mut self, command: BotAction) {
		self.queue.clear()
	}

	pub fn step(&mut self) -> StepResult {
		if self.delay > 0 {
			self.delay -= 1;
			return StepResult::Step;
		}
		if let Some(action) = self.queue.pop_front() {
			if let Some(next) = self.queue.front() {
				self.delay = next.time();
			}
			StepResult::Finished(action)
		} else {
			StepResult::Idle
		}
	}

}