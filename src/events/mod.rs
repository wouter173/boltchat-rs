use join_event::JoinEvent;
use leave_event::LeaveEvent;
use message_event::MessageEvent;

use self::{error_event::ErrorEvent, event::Event};

pub mod event;
pub mod join_event;
pub mod leave_event;
pub mod message_event;
pub mod error_event;

/// Events enum to decide which event should be de- serialized.
#[derive(Debug)]
pub enum Events {
	Join(JoinEvent),
    Leave(LeaveEvent),
    Message(MessageEvent),
    Error(ErrorEvent)
}

impl Events {
    /// This method deserializes a raw event String into the Event it should be.
    pub fn deserialize(raw: String) -> Result<Events, String> {
        
        let json: Event;
        match serde_json::from_str(&raw) {
            Ok(out) => json = out,
            Err(_) => panic!("Server disconnected."),
        }

        match json.e.t.as_str() {
            "err" => {
                let event: ErrorEvent = serde_json::from_str(&raw).unwrap();
                return Ok(Events::Error(event));
            },
            "msg" => {
                let event: MessageEvent = serde_json::from_str(&raw).unwrap();
                return Ok(Events::Message(event));
            },
            "join" => {
                let event: JoinEvent = serde_json::from_str(&raw).unwrap();
                return Ok(Events::Join(event));
            },
            "leave" => {
                let event: LeaveEvent = serde_json::from_str(&raw).unwrap();
                return Ok(Events::Leave(event));
            },
            _ => {
                return Err(String::from("Cannot decode: Event unknown."));
            }
        };
    }

    /// This method serializes an Event into the String it should be.
    pub fn serialize(event: Events) -> Result<String, String> {
        match event {
            Events::Message(e) => {
                Ok(serde_json::to_string(&e).unwrap())
            }
            Events::Join(e) => {
                Ok(serde_json::to_string(&e).unwrap())
            }
            Events::Leave(e) => {
                Ok(serde_json::to_string(&e).unwrap())
            }
            Events::Error(e) => {
                Ok(serde_json::to_string(&e).unwrap())
            }
        }
    }
}