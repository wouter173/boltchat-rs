use std::time::{SystemTime, UNIX_EPOCH};

pub fn get_timestamp() -> u64 {
	match SystemTime::now().duration_since(UNIX_EPOCH) {
		Ok(dur) => return dur.as_secs(),
		Err(e) => panic!(e)
	}
}