use super::event::{EventMeta};

use serde::{Deserialize, Serialize};

/// ErrorData struct to de- serialize the data sent with the ErrorEvent.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorData {
    pub err: String,
}

/// Error event to handle an err event from the bolt server.
#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorEvent {
    pub e: EventMeta,
    pub d: ErrorData,
}

impl ErrorEvent {
    /// Library method to create new error events for internal usage.
    pub(crate) fn new(err: String) -> Self{
        ErrorEvent {
            e: EventMeta::new("err"),
            d: ErrorData {
                err,
            }
        }
    }
}