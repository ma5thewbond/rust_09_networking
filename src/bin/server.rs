use std::env;
use std::process::exit;

use rust_09_networking::mynetmsgserver::MyNetMsgServer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let port = if args.len() < 2 {
        String::from("11111")
    } else {
        String::from(args[1].clone())
    };

    let chat_server = MyNetMsgServer::new(port).unwrap_or_else(|error| {
        eprintln!("Creating chat server failed with error {error}");
        exit(0);
    });

    chat_server.start().unwrap_or_else(|error| {
        eprintln!("Starting chat server failed with error {error}");
        exit(0);
    });
}
