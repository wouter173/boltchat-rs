use std::{io::{BufRead, BufReader, Write}, net::{TcpStream}};
use events::{event::Event, msg_event::MsgEvent};
use serde_json::{Result};
use stream::stream::Stream;
use client::client::{Client, Options};

mod events;
mod wait;
mod utils;
mod stream;
mod client;

fn main() {
    let options = Options {
        addr: String::from("localhost:3300"),
    };

    let client = Client::new(options);
    client.start();

    let stream = client.stream.tcp_stream;

    wait::sleep(0.2);
    join(&stream);
    wait::sleep(0.2);

    // let mut mutstream: TcpStream = stream.try_clone().unwrap();
    // let mut reader = BufReader::new(&mut mutstream);
    // loop {
    //     let mut buf = String::new();
    //     reader.read_line(&mut buf);
    //     print!("{}", buf);
    //     if let Err(e) = read_msg(&buf, &stream) {
    //         panic!(e);
    //     }
    // }
}

fn read_msg(event: &String, stream: &TcpStream) -> Result<()> {
    let json: Event = serde_json::from_str(&event)?;
    match json.e.t.as_str() {
        "join" => {

        },
        "msg" => {
            let event: MsgEvent = serde_json::from_str(&event)?;
            if event.msg.body == String::from("!bruh") {
                send_msg(stream, String::from("bruh"));
            }
        },
        _ => {
            return Ok(());
        }
    }

    Ok(())
}

fn send_msg(mut stream: &TcpStream, body: String) -> Result<()> {
    let event = MsgEvent::new(String::from("bruhbot"), body);

    let json = serde_json::to_string(&event)?;

    if let Err(e) = stream.write(json.as_bytes()) {
        panic!(e);
    }

    Ok(())
}

fn join(mut stream: &TcpStream) -> Result<()> {

    let event = events::join_event::JoinEvent {
        e: events::event::EventMeta {
            t: String::from("join"),
            c: 1611670138
        },
        user: events::event::User {
            nick: String::from("bruhbot")
        }
    };

    let json = serde_json::to_string(&event)?;

    // println!("{}", json);

    if let Err(e) = stream.write(json.as_bytes()) {
        panic!(e);
    }

    Ok(())
}
