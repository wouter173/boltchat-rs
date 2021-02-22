use serde::{Deserialize, Serialize};

use super::event::{EventMeta, Msg, User};

/// Messageevent struct to encode and decode the message events.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
	pub e: EventMeta,
	pub msg: Msg,
}

impl MessageEvent {
	/// This method can be called to create a new Message event.
	pub fn new(nick: String, body: String) -> MessageEvent {
		MessageEvent {
			e: EventMeta::new("msg"),
		    msg: Msg {
				body,
				sent: 0,
				user: User {
					nick
				}
			},
		}
	}
}