use crate::Stream;

pub struct Options {
	pub addr: String
}

#[derive(Debug)]
pub struct Client {
	pub stream: Stream,
	//TODO make private
}

impl Client {
	pub fn new(options: Options) -> Self {
		let stream = Stream::new(options.addr);
		
		Client {
			stream,
		}
	}

	pub fn start(&self) {
		self.stream.start();
	}
}