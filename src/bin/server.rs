use lib::deserialize_msg;
//use lib::mynetmsg::MyNetMsg;
use std::{
    collections::HashMap,
    error::Error,
    io::{self, Read},
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    sync::Mutex,
    thread::{self, JoinHandle},
    time::Duration,
};

trait Substr {
    fn substring(&self, from: usize, len: usize) -> String;
}

impl Substr for String {
    fn substring(&self, from: usize, len: usize) -> String {
        return self.chars().skip(from).take(len).collect();
    }
}

use once_cell::sync::Lazy;

static CLIENT_COUNTER: Lazy<Mutex<i32>> = Lazy::new(|| Mutex::new(0));
// because of the port selection
static LOCAL_ADDRESS: Lazy<Mutex<SocketAddr>> =
    Lazy::new(|| Mutex::new(SocketAddr::from_str("127.0.0.1:11111").unwrap()));
fn main() {
    *CLIENT_COUNTER.lock().unwrap() = 0;
    let mut waiting = true;
    println!("-- MyNetMsg::Chat server started, awaiting connections");
    let server = TcpListener::bind("0.0.0.0:11111").unwrap();
    //*LOCAL_ADDRESS.lock().unwrap() = server.local_addr().unwrap();

    let mut clients: HashMap<SocketAddr, TcpStream> = HashMap::new();
    for stream in server.incoming() {
        let client_count = *CLIENT_COUNTER.lock().unwrap();
        if !waiting && client_count == 0 {
            break;
        }
        waiting = false;
        *CLIENT_COUNTER.lock().unwrap() += 1;
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.insert(addr, stream);
        println!("-- Client connected, number of clients: {}", clients.len());
        let strm = clients.get(&addr).unwrap().try_clone().unwrap();

        //handle_client(strm, addr).unwrap();
        let _ = thread::spawn(move || handle_client(strm, addr));
        // clients.remove(&addr);
        // println!("Client disconnected, number of clients: {}", clients.len());
    }

    println!("-- 0 clients left, shutting down server.");
}

fn handle_client(
    mut stream: TcpStream,
    addr: SocketAddr,
) -> Result<SocketAddr, Box<dyn Error + Send>> {
    loop {
        let mut len_b: [u8; 4] = [0u8; 4];
        let res = stream.read_exact(&mut len_b);
        match res {
            Err(error) => {
                eprintln!("-- Client disconnected ({error})");
                *CLIENT_COUNTER.try_lock().unwrap() -= 1;
                if *CLIENT_COUNTER.try_lock().unwrap() == 0 {
                    TcpStream::connect(*LOCAL_ADDRESS.try_lock().unwrap()).unwrap();
                }
                return Ok(addr);
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
        } else {
            println!("{}: {}", msg.sender_name, msg.text);
        }
    }

    *CLIENT_COUNTER.try_lock().unwrap() -= 1;
    if *CLIENT_COUNTER.try_lock().unwrap() == 0 {
        TcpStream::connect(*LOCAL_ADDRESS.try_lock().unwrap()).unwrap();
    }
    return Ok(addr);
}
