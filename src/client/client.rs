use std::path::Path;

use crate::{events::{Events, join_event::JoinEvent, message_event::MessageEvent}, pgp::key_pair::KeyPair, stream::stream::Stream};

use super::options::Options;

/// Client struct with options and stream.
pub struct Client {
	pub options: Options,
	stream: Stream,
	keypair: KeyPair
}

impl Client {
	/// Construct a new Client with Options.
	/// The Client will load a pgp keypair from the given options.path or generate new random ones on options.path.
	pub fn new(options: Options) -> Self {
		let mut stream = Stream::new(options.addr.as_str());
		let keypair: KeyPair;

		//check if path exists otherwise generate a new keypair
		if Path::new(&options.path).exists() {
			//TODO implement the password field in Options
			keypair = KeyPair::load_keys(&options.path, String::from(""));
		} else {
			//TODO implement the password field in Options
			println!("No keys found on the `options.path`, generating new ones.");
			keypair = KeyPair::new(options.nick.as_str(), String::from(""));
			keypair.save_secret_key(&options.path);
		}

		//lmao this is shitass code.
		stream.send(Events::serialise(
			Events::Join( JoinEvent::new(
				options.nick.as_str(),
				keypair.armor_public_key(),
			))
		).unwrap());

		Client {
			options,
			stream,
			keypair
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
		let event = MessageEvent::new(self.options.nick.as_str().to_string(), msg.clone(), self.keypair.armor_message_signature(msg));
		let json = Events::serialise(Events::Message(event)).unwrap();
		self.stream.send(json);
	}
}