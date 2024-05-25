//use serde_derive::{Deserialize, Serialize};
use std::{error::Error, io::Write, net::TcpStream};

use mynetmsg::MyNetMsg;

pub mod mynetmsg;

pub fn serialize_msg(msg: mynetmsg::MyNetMsg) -> Result<Vec<u8>, Box<dyn Error>> {
    return Ok(serde_cbor::to_vec(&msg)?);
}

pub fn deserialize_msg(data: Vec<u8>) -> Result<mynetmsg::MyNetMsg, Box<dyn Error>> {
    return Ok(serde_cbor::from_slice(&data)?);
}

pub fn send_message(mut stream: &TcpStream, msg: MyNetMsg) {
    let ser_msg = serialize_msg(msg).unwrap();
    let len = ser_msg.len() as u32;
    stream.write(&len.to_be_bytes()).unwrap();
    stream.write_all(ser_msg.as_ref()).unwrap();
}
