use serde_derive::{Deserialize, Serialize};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MyNetMsg {
    pub msg_type: MyMsgType,
    pub text: String,
    pub file: Vec<u8>,
    pub file_name: String,
    pub sender: Uuid,
    pub sender_name: String,
}

impl MyNetMsg {
    pub fn builder(sender_name: String) -> MyNetMsg {
        let msg = Self {
            msg_type: MyMsgType::Text,
            text: String::new(),
            file: Vec::new(),
            file_name: String::new(),
            sender: Uuid::new_v4(),
            sender_name: sender_name,
        };
        return msg;
    }

    pub fn new_text(&self, content: String) -> Result<MyNetMsg, Box<dyn Error>> {
        let msg = Self {
            msg_type: MyMsgType::Text,
            text: content,
            file: Vec::new(),
            file_name: String::new(),
            sender: self.sender,
            sender_name: self.sender_name.clone(),
        };
        return Ok(msg);
    }

    pub fn new_file(&self, path: String) -> Result<MyNetMsg, Box<dyn Error>> {
        let msg = Self {
            msg_type: MyMsgType::File,
            text: String::new(),
            file: Self::get_file_data(&path)?,
            file_name: Self::get_file_name(&path),
            sender: self.sender,
            sender_name: self.sender_name.clone(),
        };
        return Ok(msg);
    }

    pub fn new_image(&self, path: String) -> Result<MyNetMsg, Box<dyn Error>> {
        let msg = Self {
            msg_type: MyMsgType::Image,
            text: String::new(),
            file: Self::get_file_data(&path)?,
            file_name: Self::get_file_name(&path).to_string(),
            sender: self.sender,
            sender_name: self.sender_name.clone(),
        };
        return Ok(msg);
    }

    pub fn quit_msq(content: String) -> MyNetMsg {
        let msg = Self {
            msg_type: MyMsgType::Text,
            text: content,
            file: Vec::new(),
            file_name: String::new(),
            sender: Uuid::new_v4(),
            sender_name: String::new(),
        };
        return msg;
    }

    pub fn store_file(&self, path: &Path) -> Result<(), Box<dyn Error>> {
        if !Path::new("images").exists() {
            fs::create_dir("images")?;
        }
        if !Path::new("files").exists() {
            fs::create_dir("files")?;
        }
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(path.join(&self.file_name))?;

        f.write(&self.file)?;

        return Ok(());
    }

    fn get_file_name(path: &str) -> String {
        let path: &Path = Path::new(path.trim());
        return path.file_name().unwrap().to_str().unwrap().to_string();
    }

    fn get_file_data(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
        let path = Path::new(path.trim());
        let mut f = File::open(path)?;
        let metadata = fs::metadata(path)?;
        let mut buffer = vec![0; metadata.len() as usize];
        f.read(&mut buffer)?;

        return Ok(buffer);
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MyMsgType {
    Text,
    File,
    Image,
}
