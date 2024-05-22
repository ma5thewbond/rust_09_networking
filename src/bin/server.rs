use lib::deserialize_msg;
//use lib::mynetmsg::MyNetMsg;
use std::{io::Read, net::TcpListener};

fn main() {
    let server = TcpListener::bind("0.0.0.0:11111").unwrap();
    'forcycle: for stream in server.incoming() {
        let mut stream = stream.unwrap();
        loop {
            println!("Receiving message from client");
            let mut len_b: [u8; 4] = [0u8; 4];
            let res = stream.read_exact(&mut len_b);
            match res {
                Err(error) => {
                    println!("Error reading network stream: {error}");
                    break 'forcycle;
                }
                _ => {}
            }
            let len: usize = u32::from_be_bytes(len_b) as usize;
            println!("Size of the incomming message is: {len}");
            let mut buffer: Vec<u8> = vec![0u8; len];
            stream.read_exact(&mut buffer).unwrap();
            let msg = deserialize_msg(buffer).unwrap();
            println!("message: {}", msg.text);
            println!("mesage received");
            if msg.text.trim() == ".quit" {
                println!("bye");
                break 'forcycle;
            }
        }
    }
    println!("Exited from connection");
}
