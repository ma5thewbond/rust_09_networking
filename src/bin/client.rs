use lib::mynetmsg::MyNetMsg;
use lib::serialize_msg;
use std::{io::Write, net::TcpStream};

fn main() {
    println!("Type your name:");
    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();
    user_name = user_name.trim().into();
    let message_builder = MyNetMsg::builder(user_name);
    let stream = TcpStream::connect("127.0.0.1:11111").expect("Cannot connect");
    while send_custom_message(&stream, &message_builder) {}
    println!("Bye from client");
    // send_message("Hello from client\n", &stream);
    // send_message("One more\n", &stream);
    // send_message(".quit", &stream);
}

// fn send_message(message: &str, mut stream: &TcpStream) {
//     println!("Sending message to server");
//     let msg = String::from(message);
//     let len = msg.len() as u32;
//     stream.write(&len.to_be_bytes()).unwrap();
//     stream.write(msg.as_bytes()).unwrap();
//     println!("Message sent");
// }

fn send_custom_message(mut stream: &TcpStream, mb: &MyNetMsg) -> bool {
    println!("Type message for the server");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().into();
    let exit = !input.starts_with(".quit");
    let msg = mb.new_text(input);
    let ser_msg = serialize_msg(msg).unwrap();
    let len = ser_msg.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(ser_msg.as_ref()).unwrap();
    println!("Message sent");
    return exit;
}
