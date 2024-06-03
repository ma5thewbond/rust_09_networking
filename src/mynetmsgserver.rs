use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, TcpStream},
    str::FromStr,
    sync::{Arc, RwLock},
    thread,
};

use flume::{Receiver, Sender};

use crate::{mynetmsg::MyNetMsg, read_message, send_message, Qresult, Qsendresult};

pub struct MyNetMsgServer {
    clients: Arc<RwLock<HashMap<SocketAddr, TcpStream>>>,
    port: String,
}

impl MyNetMsgServer {
    pub fn new(port: String) -> Qresult<Self> {
        let server = Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            port: port,
        };
        Ok(server)
    }

    pub fn start(&self) -> Qresult<()> {
        let mut waiting = true;
        println!("-- MyNetMsg::Chat server started, awaiting connections");
        let server_read = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        let clients2 = Arc::clone(&self.clients);
        let (send_msg, rcv_msg) = flume::unbounded::<(SocketAddr, MyNetMsg)>();
        let thandle = thread::spawn(move || send_to_all(rcv_msg, &clients2));

        for stream in server_read.incoming() {
            // send quit ping goes here and check this
            let client_count = self.clients.read().unwrap().len();
            if !waiting && client_count == 0 {
                break;
            }
            waiting = false;

            let send_msg_cln = send_msg.clone();
            let tclients = Arc::clone(&self.clients);
            let cc = MyNetMsgClientContext {
                clients: tclients,
                port: self.port.clone(),
                stream: stream.unwrap(),
            };
            let _ = thread::spawn(move || cc.handle_client(send_msg_cln));
        }
        thandle.join().unwrap();
        println!("-- 0 clients left, shutting down server.");

        Ok(())
    }
}

fn send_to_all(
    rcv_msg: Receiver<(SocketAddr, MyNetMsg)>,
    clients: &Arc<RwLock<HashMap<SocketAddr, TcpStream>>>,
) {
    loop {
        let (addr, msg) = rcv_msg.recv().unwrap(); // wait for the command from the input thread
        if msg.text == ".quit" {
            break;
        }

        let cl = Arc::clone(&clients);
        for (_, stream) in cl.read().unwrap().iter() {
            if stream.peer_addr().unwrap() == addr {
                continue;
            }
            send_message(&stream, msg.clone()).unwrap_or_else(|error| {
                eprintln!("Sending message failed with error: {error}");
            });
        }
    }
}

struct MyNetMsgClientContext {
    port: String,
    clients: Arc<RwLock<HashMap<SocketAddr, TcpStream>>>,
    stream: TcpStream,
}

impl MyNetMsgClientContext {
    pub fn handle_client(
        &self,
        send_msg: Sender<(SocketAddr, MyNetMsg)>,
    ) -> Qsendresult<SocketAddr> {
        let addr = self.stream.peer_addr().unwrap();
        self.clients
            .write()
            .unwrap()
            .insert(addr.clone(), self.stream.try_clone().unwrap());
        println!(
            "-- Client connected, number of clients: {}",
            self.clients.read().unwrap().len()
        );

        loop {
            let msg = read_message(&mut &self.stream);
            match msg {
                Ok(message) => {
                    if message.text.trim() == ".quit" {
                        println!("-- {} disconnected", message.sender_name);
                        self.clients.write().unwrap().remove(&addr);
                        if self.clients.read().unwrap().len() == 0 {
                            self.send_quit_ping(send_msg);
                        }
                        break;
                    } else {
                        message.display();
                        send_msg.send((addr, message)).unwrap();
                    }
                }
                Err(error) => {
                    eprintln!("Error reading message: {error}");
                    if self.clients.read().unwrap().len() == 0 {
                        self.send_quit_ping(send_msg);
                        break;
                    }
                }
            }
        }
        return Ok(addr);
    }

    fn send_quit_ping(&self, send_msg: Sender<(SocketAddr, MyNetMsg)>) {
        TcpStream::connect(format!("127.0.0.1:{}", self.port)).unwrap();
        send_msg
            .send((
                SocketAddr::from_str(&format!("127.0.0.1:{}", self.port)).unwrap(),
                MyNetMsg::quit_msq(String::from(".quit")),
            ))
            .unwrap();
    }
}
