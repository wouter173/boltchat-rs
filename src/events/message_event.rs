use serde::{Deserialize, Serialize};

use super::event::{EventMeta,};

/// User struct to de- serialise a message user.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageUser {
	pub nick: String,
}

/// Msg struct to de- serialise the msg.
#[derive(Serialize, Deserialize, Debug)]
pub struct Msg {
	pub body: String,
	pub sig: String,
	pub user: MessageUser,
}
/// MessageData struct to de- serialise the data sent with the MessageEvent.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageData {
	pub msg: Msg,
}

/// Messageevent struct to encode and decode the message events.
#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
	pub e: EventMeta,
	pub d: MessageData
}

impl MessageEvent {
	/// This method can be called to create a new Message event.
	pub fn new(nick: String, body: String, sig: String) -> MessageEvent {
		MessageEvent {
			e: EventMeta::new("msg"),
		    d: MessageData {
				msg: Msg {
					body,
					sig,
					user: MessageUser {
						nick,
					}
				}
			},
		}
	}
}