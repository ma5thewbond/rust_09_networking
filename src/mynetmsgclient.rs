use std::path::Path;
use std::{net::TcpStream, thread};

use crate::mynetmsg::{MyMsgType, MyNetMsg};
use crate::{prompt, read_message, read_trim_input, send_message, Qresult, Qsendresult};

pub struct MyNetMsgClient {
    pub client_name: String,
    pub address: String,
    pub message_builder: MyNetMsg,
}

impl MyNetMsgClient {
    pub fn new(addr: String) -> Qresult<Self> {
        let user_name = read_trim_input("Type your name:").unwrap_or(String::from("anonymous"));
        let client = Self {
            client_name: user_name.trim().into(),
            address: addr,
            message_builder: MyNetMsg::builder(user_name.clone()),
        };
        Ok(client)
    }

    pub fn start(&mut self) -> Qresult<()> {
        let stream = TcpStream::connect(String::from(self.address.clone()))?;
        println!(
            "-- {} connected to server, type your message",
            self.client_name
        );
        let rcv_strm = stream.try_clone().unwrap();
        let _ = thread::spawn(move || handle_received_messages(&rcv_strm));

        while self.handle_custom_message(&stream) {}
        println!("-- Bye from client");

        Ok(())
    }

    fn handle_custom_message(&self, stream: &TcpStream) -> bool {
        prompt();

        let input = read_trim_input("").unwrap_or_else(|error| {
            eprintln!("Reading user input failed with error: {error}");
            return "".into();
        });
        if input.len() == 0 {
            return true;
        }
        let cont = !input.starts_with(".quit");
        let message_data = input.split_once(' ');
        let msg = match message_data {
            Some((".text", value)) => self.message_builder.new_text(String::from(value)),
            Some((".file", value)) => self.message_builder.new_file(String::from(value)),
            Some((".image", value)) => self.message_builder.new_image(String::from(value)),
            _ => self.message_builder.new_text(input),
        };
        match msg {
            Err(error) => eprintln!("Error loading file: {error}"),
            Ok(message) => send_message(&stream, message).unwrap_or_else(|error| {
                eprintln!("Sending message failed with error: {error}");
            }),
        }
        return cont;
    }
}

fn handle_received_messages(rcv_strm: &TcpStream) -> Qsendresult<()> {
    loop {
        let msg = read_message(rcv_strm);
        match msg {
            Ok(message) => {
                message.display();
                match message.msg_type {
                    MyMsgType::Text => {}
                    MyMsgType::File => {
                        message
                            .store_file(&Path::new(".\\files"))
                            .unwrap_or_else(|error| {
                                eprintln!("Saving file failed with error: {error}");
                            });
                    }
                    MyMsgType::Image => {
                        message
                            .store_file(&Path::new(".\\images"))
                            .unwrap_or_else(|error| {
                                eprintln!("Saving image failed with error: {error}");
                            });
                    }
                };
                prompt();
            }
            Err(error) => {
                eprintln!("Error reading message: {error}");
                break;
            }
        }
    }

    Ok(())
}
