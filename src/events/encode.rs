use serde_json::{Result};

use crate::client::{client::Client};

use super::{event::Event, join_event::JoinEvent, leave_event::LeaveEvent, message_event::MessageEvent};

pub fn encode(raw: String, client: &Client) -> Result<()>{
	let json: Event = serde_json::from_str(&raw)?;
    match json.e.t.as_str() {
        "join" => {
			let event: JoinEvent = serde_json::from_str(&raw)?;
			client.get_handler().join(event);
        },
        "msg" => {
			let event: MessageEvent = serde_json::from_str(&raw)?;
           	client.get_handler().message(event);
        },
		"leave" => {
			let event: LeaveEvent = serde_json::from_str(&raw)?;
			client.get_handler().leave(event);
		},
        _ => {
            return Ok(());
        }
    };

	Ok(())
}