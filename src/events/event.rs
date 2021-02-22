use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

/// Eventmeta that is appended to every event.
#[derive(Serialize, Deserialize, Debug)]
pub struct EventMeta {
    pub t: String,
    pub c: u64,
}

impl EventMeta {
	pub fn new(t: &str) -> Self {
		EventMeta {
			t: String::from(t),
			c: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
		}
	}
}

/// User struct to deserialise the user.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub nick: String,
}

/// Msg struct to deserialise the msg.
#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
	pub sent: i16,
	pub body: String,
	pub user: User,
}

/// Event struct to template every event from.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
	pub e: EventMeta
}