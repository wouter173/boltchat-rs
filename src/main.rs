use events::{join_event::JoinEvent, leave_event::LeaveEvent, message_event::MessageEvent};
use stream::stream::Stream;
use client::{event_handler::EventHandler, client::{Client, Options}};

mod events;
mod utils;
mod stream;
mod client;

struct Handler;

impl EventHandler for Handler {
    fn message(&self, event: MessageEvent) {
        println!("msg: [{}] {}", event.msg.user.nick, event.msg.body);
    }

    fn join(&self, event: JoinEvent) {
        println!("join: {}", event.user.nick);
    }

    fn leave(&self, event: LeaveEvent) {
        println!("leave: {}", event.user.nick);
    }
}

fn main() {
    let options = Options {
        addr: String::from("127.0.0.1:3300"),
        nick: String::from("bruhbot"),
    };

    let mut client = Client::new(options);
    client.set_handler(Handler);
    client.start();
}
