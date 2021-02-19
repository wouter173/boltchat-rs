use serde::{Deserialize, Serialize};
use crate::utils::timestamp::get_timestamp;

use super::event::{EventMeta, Msg, User};

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageEvent {
	pub e: EventMeta,
	pub msg: Msg,
}

impl MessageEvent {
	pub fn new(nick: String, body: String) -> MessageEvent {
		MessageEvent {
			e: EventMeta {
				t: String::from("msg"),
				c: get_timestamp(),
			},
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