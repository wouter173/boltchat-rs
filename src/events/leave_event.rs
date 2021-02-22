use super::event::{EventMeta, User};
use serde::{Deserialize, Serialize};

/// Leaveevent struct that decodes leave events from other users.
#[derive(Serialize, Deserialize, Debug)]
pub struct LeaveEvent {
	pub e: EventMeta,
	pub user: User,
}