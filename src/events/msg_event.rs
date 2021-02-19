use serde::{Deserialize, Serialize};
use crate::utils::timestamp::get_timestamp;

use super::event::{EventMeta, Msg, User};

#[derive(Serialize, Deserialize, Debug)]
pub struct MsgEvent {
	pub e: EventMeta,
	pub msg: Msg,
}

impl MsgEvent {
	pub fn new(nick: String, body: String) -> MsgEvent {
		MsgEvent {
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