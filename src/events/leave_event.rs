use super::event::{EventMeta, User};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveEvent {
	pub e: EventMeta,
	pub user: User,
}