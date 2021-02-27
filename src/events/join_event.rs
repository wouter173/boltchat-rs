use serde::{Deserialize, Serialize};

use super::event::{EventMeta, User};
/// JoinData struct to de- serialise the data sent with the JoinEvent.
#[derive(Serialize, Deserialize, Debug)]
pub struct JoinData {
    pub user: User,
}

/// Joinevent struct to de- serialise.
#[derive(Serialize, Deserialize, Debug)]
pub struct JoinEvent {
    pub e: EventMeta,
    pub d: JoinData,
}

impl JoinEvent {
    /// This is a library event that gets sent everytime you connect to a bolt.
    pub fn new(nick: String, pubkey: String) -> Self {
        JoinEvent {
            e: EventMeta::new("join"),
            d: JoinData {
                user: User {
                    nick,
                    pubkey
                },
            }
        }
    }
}