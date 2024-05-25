use lib::{mynetmsg::MyNetMsg, send_message};
use std::net::TcpStream;

fn main() {
    println!("Type your name:");
    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();
    user_name = user_name.trim().into();
    let message_builder = MyNetMsg::builder(user_name);
    let stream = TcpStream::connect("127.0.0.1:11111").expect("Cannot connect");
    while handle_custom_message(&stream, &message_builder) {}
    println!("Bye from client");
}

pub fn handle_custom_message(stream: &TcpStream, mb: &MyNetMsg) -> bool {
    println!("Type message for the server");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().into();
    let exit = !input.starts_with(".quit");
    let msg = mb.new_text(input);
    send_message(stream, msg);
    println!("Message sent");
    return exit;
}
