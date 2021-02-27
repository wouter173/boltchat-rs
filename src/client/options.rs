/// The options for the client.
/// addr is the ipaddress or connection string to connect to.
/// nick is the nickname the client has in a bolt server.
/// path is the path to the pgp secret key storage file.
/// password is the password with which to sign the secret key.
pub struct Options {
	pub addr: String,
	pub nick: String, 
	pub path: String,
	pub password: String,
}

impl Options {
	pub fn new(addr: String, nick: String, path: String) -> Self {
		Options {
			addr,
			nick,
			path,
			password: String::from(""),
		}
	}
}