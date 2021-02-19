use serde::{Deserialize, Serialize};

use super::event::{EventMeta, User};

#[derive(Serialize, Deserialize, Debug)]
pub struct JoinEvent {
    pub e: EventMeta,
    pub user: User,
}