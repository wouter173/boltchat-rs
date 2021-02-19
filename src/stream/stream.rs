use std::{io::{BufRead, BufReader}, net::TcpStream};

#[derive(Debug)]
pub struct Stream {
	pub tcp_stream: TcpStream,
	//TODO make private
}

impl Stream {
    pub fn new(addr: String) -> Self {
		Stream {
			tcp_stream: TcpStream::connect(addr).unwrap(),
		}
	}

	pub fn start(&self) {

		let mut mutstream: TcpStream = self.tcp_stream.try_clone().unwrap();
		let mut reader = BufReader::new(&mut mutstream);
		loop {
			let mut buf = String::new();
			reader.read_line(&mut buf);
			print!("{}", buf);
		}
	}
}