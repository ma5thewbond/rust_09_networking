use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct MyNetMsg {
    pub msg_type: MyMsgType,
    pub text: String,
    pub file: Vec<u8>,
    pub file_name: String,
    sender: Uuid,
}

impl MyNetMsg {
    pub fn new_text(content: String, sender: Uuid) -> MyNetMsg {
        let msg = Self {
            msg_type: MyMsgType::Text,
            text: content,
            file: Vec::new(),
            file_name: String::new(),
            sender: sender,
        };
        return msg;
    }

    pub fn new_file(file_name: String, content: Vec<u8>, sender: Uuid) -> MyNetMsg {
        let msg = Self {
            msg_type: MyMsgType::File,
            text: String::new(),
            file: content,
            file_name: file_name,
            sender: sender,
        };
        return msg;
    }

    pub fn new_image(file_name: String, content: Vec<u8>, sender: Uuid) -> MyNetMsg {
        let msg = Self {
            msg_type: MyMsgType::Image,
            text: String::new(),
            file: content,
            file_name: file_name,
            sender: sender,
        };
        return msg;
    }

    fn new_incomming(&self, msg_type: MyMsgType) -> MyNetMsg {
        let msg = Self {
            msg_type: msg_type,
            text: String::new(),
            file: Vec::new(),
            file_name: String::new(),
            sender: self.sender.clone(),
        };
        return msg;
    }

    pub fn get_incomming(&self) -> Result<MyNetMsg, Box<dyn Error>> {
        match self.msg_type {
            MyMsgType::File => {
                let inc = MyNetMsg::new_incomming(self, MyMsgType::IncomingFile);
                return Ok(inc);
            }
            MyMsgType::Image => {
                let inc = MyNetMsg::new_incomming(self, MyMsgType::IncomingImage);
                return Ok(inc);
            }
            _ => {
                return Err("Type of message doesn't have incomming variant".into());
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum MyMsgType {
    Text,
    File,
    Image,
    IncomingFile,
    IncomingImage,
}
