use serde::{Deserialize, Serialize};

use super::event::{EventMeta, User};

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinEvent {
    pub e: EventMeta,
    pub user: User,
}

impl JoinEvent {
    pub fn new(nick: &str) -> Self {
        JoinEvent {
            e: EventMeta::new("join"),
            user: User {
                nick: String::from(nick)
            },
        }
    }
}