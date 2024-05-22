use lib::mynetmsg::MyNetMsg;
use lib::serialize_msg;
use std::{io::Write, net::TcpStream};
use uuid::Uuid;

fn main() {
    let client_id = Uuid::new_v4();
    let stream = TcpStream::connect("127.0.0.1:11111").expect("Cannot connect");
    while send_custom_message(&stream, client_id) {}
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

fn send_custom_message(mut stream: &TcpStream, client_id: Uuid) -> bool {
    println!("Type message for the server");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let exit = !input.starts_with(".quit");
    let msg = MyNetMsg::new_text(input, client_id);
    let ser_msg = serialize_msg(msg).unwrap();
    let len = ser_msg.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(ser_msg.as_ref()).unwrap();
    println!("Message sent");
    return exit;
}
