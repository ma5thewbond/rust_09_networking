use flume::{Receiver, Sender};
use lib::{deserialize_msg, mynetmsg::MyNetMsg, send_message};
//use lib::mynetmsg::MyNetMsg;
use std::{
    collections::HashMap,
    error::Error,
    io::Read,
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, Mutex, RwLock},
    thread,
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
    let server_read = TcpListener::bind("0.0.0.0:11111").unwrap();
    //*LOCAL_ADDRESS.lock().unwrap() = server.local_addr().unwrap();

    let clients: Arc<RwLock<HashMap<SocketAddr, TcpStream>>> =
        Arc::new(RwLock::new(HashMap::new()));
    let clients2 = Arc::clone(&clients);
    let (send_msg, rcv_msg) = flume::unbounded::<MyNetMsg>();
    let thandle = thread::spawn(move || send_to_all(rcv_msg, &clients2));

    for stream in server_read.incoming() {
        let client_count = *CLIENT_COUNTER.lock().unwrap();
        if !waiting && client_count == 0 {
            break;
        }
        waiting = false;
        *CLIENT_COUNTER.lock().unwrap() += 1;
        let stream = stream.unwrap();
        let addr = stream.peer_addr().unwrap();
        clients.write().unwrap().insert(addr, stream);
        println!(
            "-- Client connected, number of clients: {}",
            clients.read().unwrap().len()
        );
        let strm = clients
            .read()
            .unwrap()
            .get(&addr)
            .unwrap()
            .try_clone()
            .unwrap();

        //handle_client(strm, addr).unwrap();
        let send_msg_cln = send_msg.clone();
        let _ = thread::spawn(move || handle_client(strm, addr, send_msg_cln));
        // clients.remove(&addr);
        // println!("Client disconnected, number of clients: {}", clients.len());
    }
    thandle.join().unwrap();
    println!("-- 0 clients left, shutting down server.");
}

fn handle_client(
    mut stream: TcpStream,
    addr: SocketAddr,
    send_msg: Sender<MyNetMsg>,
) -> Result<SocketAddr, Box<dyn Error + Send>> {
    loop {
        println!("Reading incomming message");
        let mut len_b: [u8; 4] = [0u8; 4];
        let res = stream.read_exact(&mut len_b);
        println!("Message red");
        match res {
            Err(error) => {
                eprintln!("-- Client disconnected ({error})");
                *CLIENT_COUNTER.try_lock().unwrap() -= 1;
                if *CLIENT_COUNTER.try_lock().unwrap() == 0 {
                    send_quit_ping(send_msg);
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
            *CLIENT_COUNTER.try_lock().unwrap() -= 1;
            let count = *CLIENT_COUNTER.try_lock().unwrap();

            if count == 0 {
                send_quit_ping(send_msg);
            }
            break;
        } else {
            println!("{}: {}", msg.sender_name, msg.text);
            send_msg.send(msg).unwrap();
        }
    }
    return Ok(addr);
}

fn send_to_all(rcv_msg: Receiver<MyNetMsg>, clients: &Arc<RwLock<HashMap<SocketAddr, TcpStream>>>) {
    loop {
        let msg = rcv_msg.recv().unwrap(); // wait for the command from the input thread
        if msg.text == ".quit" {
            break;
        }

        let cl = Arc::clone(&clients);
        for (_, stream) in cl.read().unwrap().iter() {
            send_message(&stream, msg.clone());
        }
    }
}

fn send_quit_ping(send_msg: Sender<MyNetMsg>) {
    TcpStream::connect(*LOCAL_ADDRESS.try_lock().unwrap()).unwrap();
    send_msg
        .send(MyNetMsg::quit_msq(String::from(".quit")))
        .unwrap();
}
