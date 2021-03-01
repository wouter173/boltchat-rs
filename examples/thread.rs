use std::thread;

use boltchat::{client::{client::Client, options::Options}, events::Events};

fn main() {
    let options = Options::new(
        String::from("127.0.0.1:3300"),
        String::from("pingu"),
        String::from("./secret.pgp")
    );

    //create client
    let mut client: Client = Client::new(options);

    //clone client
    let mut cl = client.clone();

    //spawn new thread for handling join and leave events.
    thread::spawn(move || {
        loop {
            match cl.receive() {
                Some(Events::Join(event)) => {
                println!("join => {}", event.d.user.nick);
                },
                Some(Events::Leave(event)) => {
                    println!("leave => {}", event.d.user.nick);
                },
                Some(_) => {}
                None => {}
            }
        }
    });

    //handle error and message on main thread
    loop {
        match client.receive() {
            Some(Events::Error(event)) => {
                println!("error => {}", event.d.err);
            },
            Some(Events::Message(event)) => {
                println!("message => [{}]: {}", event.d.msg.user.nick, event.d.msg.body);
                if event.d.msg.body == String::from("!ping") {
                    client.send_message(String::from("pong"));
                }
            },
            Some(_) => {}
            None => {}
        }
    }
}