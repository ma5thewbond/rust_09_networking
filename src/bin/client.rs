use lib::mynetmsg::MyMsgType;
use lib::{mynetmsg::MyNetMsg, read_message, send_message};
use std::io::Write;
use std::path::Path;
use std::process::exit;
use std::{env, io, thread};
use std::{error::Error, net::TcpStream};
use uuid::Uuid;

fn main() {
    let args: Vec<String> = env::args().collect();
    let address = if args.len() < 3 {
        String::from("127.0.0.1:11111")
    } else {
        String::from(format!("{}:{}", args[1], args[2]))
    };
    println!("Address: {address}");
    println!("Type your name:");
    let mut user_name = String::new();
    std::io::stdin().read_line(&mut user_name).unwrap();
    user_name = user_name.trim().into();
    let message_builder = MyNetMsg::builder(user_name.clone());
    let stream = TcpStream::connect(String::from(&address)).unwrap_or_else(|error| {
        println!("Connecting to server failed with error:\n{error}");
        exit(0);
    });
    println!("-- {user_name} connected to server, type your message");
    let rcv_strm = stream.try_clone().unwrap();
    let _ = thread::spawn(move || handle_received_messages(rcv_strm, &message_builder.sender));
    let mut cont = true;
    while cont {
        //let read_strm = Arc::clone(&stream);
        cont = handle_custom_message(&stream, &message_builder);
    }
    println!("-- Bye from client");
    exit(0);
    //let _ = handle.join().unwrap();
}

fn handle_custom_message(stream: &TcpStream, mb: &MyNetMsg) -> bool {
    print!(": ");
    io::stdout()
        .flush()
        .unwrap_or_else(|error| eprintln!("flush error: {error}"));
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input = input.trim().into();
    let cont = !input.starts_with(".quit");
    let message_data = input.split_once(' ');
    let msg = match message_data {
        Some((".text", value)) => mb.new_text(String::from(value)),
        Some((".file", value)) => mb.new_file(String::from(value)),
        Some((".image", value)) => mb.new_image(String::from(value)),
        _ => mb.new_text(input),
    };
    match msg {
        Err(error) => eprintln!("Error loading file: {error}"),
        Ok(message) => send_message(stream, message).unwrap_or_else(|error| {
            eprintln!("Sending message failed with error: {error}");
        }),
    }
    return cont;
}

fn handle_received_messages(
    mut stream: TcpStream,
    sender: &Uuid,
) -> Result<(), Box<dyn Error + Send>> {
    loop {
        let msg = read_message(&mut stream);
        match msg {
            Ok(message) => {
                if message.sender != *sender {
                    match message.msg_type {
                        MyMsgType::Text => {
                            if message.text.trim() == ".quit" {
                                println!("\n-- {} disconnected", message.sender_name);
                                break;
                            } else {
                                println!("\n{}: {}", message.sender_name, message.text);
                            }
                        }
                        MyMsgType::File => {
                            println!(
                                "\n{}: incomming file {}",
                                message.sender_name, message.file_name
                            );
                            message
                                .store_file(&Path::new(".\\files"))
                                .unwrap_or_else(|error| {
                                    eprintln!("Saving file failed with error: {error}");
                                });
                        }
                        MyMsgType::Image => {
                            println!(
                                "\n{}: incomming image {}",
                                message.sender_name, message.file_name
                            );
                            message
                                .store_file(&Path::new(".\\images"))
                                .unwrap_or_else(|error| {
                                    eprintln!("Saving image failed with error: {error}");
                                });
                        }
                    };
                    print!(": ");
                    io::stdout()
                        .flush()
                        .unwrap_or_else(|error| eprintln!("flush error: {error}"));
                }
            }
            Err(error) => {
                eprintln!("Error reading message: {error}");
                break;
            }
        }
        // else {
        //     println!("me: {}", msg.text);
        // }
    }

    return Ok(());
}
