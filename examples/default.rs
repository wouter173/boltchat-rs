use bolt::events::{Events};
use bolt::client::{client::{Client, Options}};

fn main() {
    let options = Options {
        addr: String::from("127.0.0.1:3300"),
        nick: String::from("bruhbot"),
    };

    let mut client: Client = Client::new(options);
    
    loop {
        match client.recieve() {
            Some(Events::Error(event)) => {
                println!("{:?}", event);
            },
            Some(Events::Message(event)) => {
                println!("{:?}", event);
                if event.msg.body == String::from("!bruh") {
                    client.send_message(String::from("agree"));
                }
            },
            Some(Events::Join(event)) => {
                println!("{:?}", event);
            },
            Some(Events::Leave(event)) => {
                println!("{:?}", event);
            },
            None => {

            }
        }
    }
}
