use lib::{deserialize_msg, mynetmsg::MyNetMsg, send_message};
use std::io::Read;
use std::sync::{Arc, RwLock};
use std::thread;
use std::{error::Error, net::TcpStream};
use uuid::Uuid;

fn main() {
    println!("Type your name:");
    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();
    user_name = user_name.trim().into();
    let message_builder = MyNetMsg::builder(user_name.clone());
    let stream = TcpStream::connect("127.0.0.1:11111").expect("Cannot connect");
    //let stream = Arc::new(RwLock::new(stream));
    println!("-- {user_name} connected to server");
    let rcv_strm = stream.try_clone().unwrap();
    let handle = thread::spawn(move || handle_received_messages(rcv_strm, &message_builder.sender));
    let mut cont = true;
    while cont {
        //let read_strm = Arc::clone(&stream);
        cont = handle_custom_message(&stream, &message_builder);
    }
    let _ = handle.join().unwrap();
    println!("-- Bye from client");
}

fn handle_custom_message(stream: &TcpStream, mb: &MyNetMsg) -> bool {
    println!("Type message for the server");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().into();
    let exit = !input.starts_with(".quit");
    let msg = mb.new_text(input);
    send_message(stream, msg);
    //println!("Message sent");
    return exit;
}

fn handle_received_messages(
    mut stream: TcpStream,
    sender: &Uuid,
) -> Result<(), Box<dyn Error + Send>> {
    loop {
        println!("reading incomming msg");
        let mut len_b: [u8; 4] = [0u8; 4];
        let res = stream.read_exact(&mut len_b);
        println!("msg red");
        match res {
            Err(error) => {
                eprintln!("-- Client disconnected ({error})");
                return Ok(());
            }
            _ => {}
        }
        let len: usize = u32::from_be_bytes(len_b) as usize;
        //println!("Size of the incomming message is: {len}");
        let mut buffer: Vec<u8> = vec![0u8; len];
        stream.read_exact(&mut buffer).unwrap();
        let msg = deserialize_msg(buffer).unwrap();
        if msg.text.trim() == ".quit" {
            println!("-- {} disconnected", msg.sender_name);
            break;
        } else if msg.sender != *sender {
            println!("{}: {}", msg.sender_name, msg.text);
        } else {
            println!("me: {}", msg.text);
        }
    }

    return Ok(());
}
