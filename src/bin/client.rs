use rust_09_networking::mynetmsgclient::*;
use std::env;
use std::process::exit;

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() < 3 {
        String::from("127.0.0.1:11111")
    } else {
        String::from(format!("{}:{}", args[1], args[2]))
    };

    let mut chat_client = MyNetMsgClient::new(address).unwrap_or_else(|error| {
        eprintln!("Creating chat client failed with error {error}");
        exit(0);
    });
    chat_client.start().unwrap_or_else(|error| {
        eprintln!("Starting chat client failed with error {error}");
        exit(0);
    });
}
