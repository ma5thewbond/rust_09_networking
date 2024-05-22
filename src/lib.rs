//use serde_derive::{Deserialize, Serialize};
use std::error::Error;

pub mod mynetmsg;

pub fn serialize_msg(msg: mynetmsg::MyNetMsg) -> Result<Vec<u8>, Box<dyn Error>> {
    return Ok(serde_cbor::to_vec(&msg)?);
}

pub fn deserialize_msg(data: Vec<u8>) -> Result<mynetmsg::MyNetMsg, Box<dyn Error>> {
    return Ok(serde_cbor::from_slice(&data)?);
}
