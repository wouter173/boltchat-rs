use serde::{Deserialize, Serialize};

use crate::utils::timestamp::get_timestamp;

#[derive(Serialize, Deserialize, Debug)]
pub struct EventMeta {
    pub t: String,
    pub c: u64,
}

impl EventMeta {
	pub fn new(t: &str) -> Self {
		EventMeta {
			t: String::from(t),
			c: get_timestamp()
		}
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub nick: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
	pub sent: i16,
	pub body: String,
	pub user: User,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
	pub e: EventMeta
}