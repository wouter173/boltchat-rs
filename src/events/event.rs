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

/// User struct to de- serialise the user.
/// Holds the users nick and pubkey.
/// Used in the join and leave events.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub nick: String,
	pub pubkey: String,
}

/// Event struct to decode all events from.
#[derive(Serialize, Deserialize, Debug)]
pub struct Event {
	pub e: EventMeta
}