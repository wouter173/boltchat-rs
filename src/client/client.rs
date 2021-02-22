use crate::{stream::stream::Stream, events::{Events, join_event::JoinEvent, message_event::MessageEvent}};

/// The options for the client
pub struct Options {
	pub addr: String,
	pub nick: String, 
}
/// Client struct with options and stream.
pub struct Client {
	pub options: Options,
	stream: Stream,
}

impl Client {
	/// Construct a new Client with Options.
	pub fn new(options: Options) -> Self {
		let mut stream = Stream::new(options.addr.as_str());

		//lmao this is shitass code.
		stream.send(Events::encode(Events::Join(JoinEvent::new(options.nick.as_str()))).unwrap());

		Client {
			options,
			stream,
		}
	}

	/// Recieve decoded events from the bolt server.
	pub fn recieve(&mut self) -> Option<Events> {
		loop {
			if let Some(json) = self.stream.recv_line() {
				return Some(json);
			}
		}
	}

	/// Send a message to the bolt server.
	pub fn send_message(&mut self, msg: String) {
		let event = MessageEvent::new(self.options.nick.as_str().to_string(), msg);
		let json = Events::encode(Events::Message(event)).unwrap();
		self.stream.send(json);
	}
}