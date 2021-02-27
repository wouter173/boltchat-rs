/// The options for the client.
/// addr is the ipaddress or connection string to connect to.
/// nick is the nickname the client has in a bolt server.
/// path is the path to the pgp secret key storage file.
pub struct Options {
	pub addr: String,
	pub nick: String, 
	pub path: String,
}

impl Options {
	pub fn new(addr: String, nick: String, path: String) -> Self {
		Options {
			addr,
			nick,
			path
		}
	}
}