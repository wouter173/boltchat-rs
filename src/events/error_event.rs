use super::event::{EventMeta};

use serde::{Deserialize, Serialize};

/// Error event to handle an err event from the bolt server.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorEvent {
    pub e: EventMeta,
    pub err: String,
}

impl ErrorEvent {
    /// Library method to create new error events for internal usage.
    pub(crate) fn new(err: String) -> Self{
        ErrorEvent {
            e: EventMeta::new("err"),
            err
        }
    }
}