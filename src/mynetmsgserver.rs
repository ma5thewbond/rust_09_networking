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
    msg_builder: MyNetMsg,
}

impl MyNetMsgServer {
    pub fn new(port: String) -> Qresult<Self> {
        let server = Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
            port: port,
            msg_builder: MyNetMsg::builder("Server".into()),
        };
        Ok(server)
    }

    pub fn start(&self) -> Qresult<()> {
        let mut waiting = true;
        println!("-- MyNetMsg::Chat server started, awaiting connections");
        let server_read = TcpListener::bind(format!("0.0.0.0:{}", self.port))?;
        let clients2 = Arc::clone(&self.clients);
        let (send_msg, rcv_msg) = flume::unbounded::<(Option<SocketAddr>, MyNetMsg)>();
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
                msg_builder: self.msg_builder.clone(),
            };
            let _ = thread::spawn(move || cc.handle_client(send_msg_cln));
        }
        thandle.join().unwrap();
        println!("-- 0 clients left, shutting down server.");

        Ok(())
    }
}

fn send_to_all(
    rcv_msg: Receiver<(Option<SocketAddr>, MyNetMsg)>,
    clients: &Arc<RwLock<HashMap<SocketAddr, TcpStream>>>,
) {
    loop {
        let (addr, msg) = rcv_msg.recv().unwrap(); // wait for the command from the input thread
        if msg.text == ".quit" {
            break;
        }

        let cl = Arc::clone(&clients);

        for (cl_addr, stream) in cl.read().unwrap().iter() {
            // if addr is provided, do not send message back to sender, otherwise it is server status message to everyone
            if addr.is_some() && *cl_addr == addr.unwrap() {
                // stream.peer_addr().unwrap()
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
    msg_builder: MyNetMsg,
}

impl MyNetMsgClientContext {
    pub fn handle_client(
        &self,
        send_msg: Sender<(Option<SocketAddr>, MyNetMsg)>,
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

        let status_msg = self
            .msg_builder
            .new_text(format!(
                "-- Client connected, number of clients: {}",
                self.clients.read().unwrap().len()
            ))
            .unwrap();
        send_msg.send((Some(addr), status_msg)).unwrap();

        loop {
            let msg = read_message(&mut &self.stream);
            match msg {
                Ok(mut message) => {
                    if message.text.trim() == ".quit" {
                        println!("-- {} disconnected", message.sender_name);
                        self.clients.write().unwrap().remove(&addr);
                        if self.clients.read().unwrap().len() == 0 {
                            self.send_quit_ping(send_msg);
                        }
                        break;
                    } else {
                        message.display();
                        let convres = message.convert_to_png();
                        if convres.is_err() {
                            eprintln!(
                                "Conversion to png failed with error: {}",
                                convres.unwrap_err()
                            );

                            let errmsg = self
                                .msg_builder
                                .new_text(
                                    "Image has unsupported format or file is corrupted".into(),
                                )
                                .unwrap();

                            for (_, stream) in self.clients.read().unwrap().iter() {
                                send_message(&stream, errmsg.clone()).unwrap_or_else(|error| {
                                    eprintln!("Sending error message failed with error: {error}");
                                });
                            }
                        } else {
                            send_msg.send((Some(addr), message)).unwrap();
                        }
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

    fn send_quit_ping(&self, send_msg: Sender<(Option<SocketAddr>, MyNetMsg)>) {
        TcpStream::connect(format!("127.0.0.1:{}", self.port)).unwrap();
        send_msg
            .send((
                Some(SocketAddr::from_str(&format!("127.0.0.1:{}", self.port)).unwrap()),
                MyNetMsg::quit_msq(String::from(".quit")),
            ))
            .unwrap();
    }
}
