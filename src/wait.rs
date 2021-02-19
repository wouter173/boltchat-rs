use std::process::Command;

pub fn sleep(time: f32) {
	if let Err(e) = Command::new("sleep").arg(time.to_string()).spawn().unwrap().wait() {
		panic!(e);
	}
}