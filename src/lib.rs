//use serde_derive::{Deserialize, Serialize};
use mynetmsg::MyNetMsg;
use std::io::Read;
use std::{error::Error, io::Write, net::TcpStream};

pub mod mynetmsg;
pub mod mynetmsgclient;
pub mod mynetmsgserver;

pub fn serialize_msg(msg: mynetmsg::MyNetMsg) -> Result<Vec<u8>, Box<dyn Error>> {
    return Ok(serde_cbor::to_vec(&msg)?);
}

pub fn deserialize_msg(data: Vec<u8>) -> Result<mynetmsg::MyNetMsg, Box<dyn Error>> {
    return Ok(serde_cbor::from_slice(&data)?);
}

pub fn send_message(mut stream: &TcpStream, msg: MyNetMsg) -> Result<(), Box<dyn Error>> {
    let ser_msg = serialize_msg(msg)?;
    let len = ser_msg.len() as u32;
    stream.write(&len.to_be_bytes())?;
    stream.write_all(ser_msg.as_ref())?;

    return Ok(());
}

pub fn read_message(mut stream: &TcpStream) -> Result<MyNetMsg, Box<dyn Error>> {
    let mut len_b: [u8; 4] = [0u8; 4];
    let res = stream.read_exact(&mut len_b);
    match res {
        Err(error) => {
            eprintln!("-- Client disconnected ({error})");
            return Err(format!("-- Client disconnected ({error})").into());
        }
        _ => {}
    }
    let len: usize = u32::from_be_bytes(len_b) as usize;
    let mut buffer: Vec<u8> = vec![0u8; len];
    stream.read_exact(&mut buffer)?;
    let msg = deserialize_msg(buffer);
    return msg;
}

pub fn read_trim_input(prompt: &str) -> Qresult<String> {
    if prompt.len() > 0 {
        println!("{prompt}");
    }
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    return Ok(input.trim().into());
}

pub fn prompt() {
    print!(": ");
    std::io::stdout()
        .flush()
        .unwrap_or_else(|error| eprintln!("flush error: {error}"));
}

trait Substr {
    fn substring(&self, from: usize, len: usize) -> String;
}

impl Substr for String {
    fn substring(&self, from: usize, len: usize) -> String {
        return self.chars().skip(from).take(len).collect();
    }
}

pub type Qresult<T> = Result<T, Box<dyn Error>>;
pub type Qsendresult<T> = Result<T, Box<dyn Error + Send>>;
