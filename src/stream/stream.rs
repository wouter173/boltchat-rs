use std::{io::{BufRead, BufReader, Write}, net::TcpStream};

use crate::client::client::Client;
use crate::events::encode::encode;

#[derive(Debug)]
pub struct Stream {
	pub tcp_stream: TcpStream,
	//TODO make private
}

impl Stream {
    pub fn new(addr: &str) -> Self {
		Stream {
			tcp_stream: TcpStream::connect(addr).unwrap(),
		}
	}

	pub fn start(&self, client: &Client) {

		let mut mutstream: TcpStream = self.tcp_stream.try_clone().unwrap();
		let mut reader = BufReader::new(&mut mutstream);
		loop {
			let mut buf = String::new();
			if let Err(e) = reader.read_line(&mut buf) {
				panic!(e);
			};

			if let Err(e) = encode(buf, &client) {
				panic!(e);
			};
		}
	}

	pub fn send(&mut self, json: String) {
		if let Err(e) = self.tcp_stream.write(json.as_bytes()) {
			panic!(e);
		}
	}
}