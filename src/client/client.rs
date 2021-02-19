use std::sync::Arc;

use crate::{Stream, events::join_event::JoinEvent};

use super::event_handler::{EventHandler};

pub struct Options {
	pub addr: String,
	pub nick: String,
}

pub struct Client {
	options: Options,
	stream: Stream,
	handler: Option<Arc<dyn EventHandler>>,
}

impl Client {
	// Construct a new Client with Options.
	pub fn new(options: Options) -> Self {
		let stream = Stream::new(options.addr.as_str());
		
		Client {
			options,
			stream,
			handler: None,
		}
	}

	// Start the client and send the join event.
	pub fn start(&mut self) {
		let json = serde_json::to_string(&JoinEvent::new(self.options.nick.as_str())).unwrap();
		self.stream.send(json);
		self.stream.start(self);
	}

    // Set the client's event handler.
    pub fn set_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.handler = Some(Arc::new(handler));
    }

	// Get the client's event handler.
	pub fn get_handler(&self) -> &Arc<dyn EventHandler> {
		match &self.handler {
		    Some(handler) => return handler,
		    None => panic!("no event handler mounted!")
		}
	}
}