use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EventMeta {
    pub t: String,
    pub c: u64,
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