use serde::{Deserialize, Serialize};

use super::event::{EventMeta, User};

/// Joinevent struct to encode and decode.
#[derive(Serialize, Deserialize, Debug)]
pub struct JoinEvent {
    pub e: EventMeta,
    pub user: User,
}

impl JoinEvent {
    /// This is a library event that gets sent everytime you connect to a bolt.
    pub fn new(nick: &str) -> Self {
        JoinEvent {
            e: EventMeta::new("join"),
            user: User {
                nick: String::from(nick)
            },
        }
    }
}