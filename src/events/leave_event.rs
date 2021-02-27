use super::event::{EventMeta, User};
use serde::{Deserialize, Serialize};

/// LeaveData struct to deserialize the data sent with the LeaveEvent.
#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveData {
	pub user: User,
}

/// LeaveEvent struct that deserializes leave events from other users.
#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveEvent {
	pub e: EventMeta,
	pub d: LeaveData,
}