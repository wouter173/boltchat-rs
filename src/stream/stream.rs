use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

use crate::{events::Events};

/// Stream struct with a tcpstream.
#[derive(Debug)]
pub struct Stream {
	tcp_stream: TcpStream,
}

impl Stream {
	/// This is a library method that is called to create a new Stream.
    pub(crate) fn new(addr: &str) -> Self {
		Stream {
			tcp_stream: TcpStream::connect(addr).unwrap(),
		}
	}

	/// This is a library method that is called to get an event from the bolt server
	pub(crate) fn recv_line(&self) -> Option<Events> {

		let mut mutstream: TcpStream = self.tcp_stream.try_clone().unwrap();
		let mut reader = BufReader::new(&mut mutstream);

		loop {
			let mut buf = String::new();
			if let Err(e) = reader.read_line(&mut buf) {
				panic!(e);
			};

			match Events::decode(buf) {
				Ok(event) => return Some(event),
				Err(err) => println!("{}", err),
			}
		}
	}


	/// This is a library method to send a raw event to the bolt server.
	pub(crate) fn send(&mut self, json: String) {
		self.tcp_stream.write(json.as_bytes()).unwrap();
	}
}