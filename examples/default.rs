use boltchat::events::{Events};
use boltchat::client::{options::Options, client::Client};

fn main() {
    let options = Options::new(
        String::from("127.0.0.1:3300"),
        String::from("pingu"),
        String::from("./secret.pgp")
    );

    let mut client: Client = Client::new(options);

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
            Some(Events::Join(event)) => {
                println!("join => {}", event.d.user.nick);
            },
            Some(Events::Leave(event)) => {
                println!("leave => {}", event.d.user.nick);
            },
            None => {

            }
        }
    }
}
