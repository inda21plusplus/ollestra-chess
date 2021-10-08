use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::Mutex;
use std::sync::{mpsc, Arc};
use std::thread;

const IP: &str = "127.0.0.1";
const PORT: &str = "1337";

pub struct Server {
    pub listener: TcpListener,
}

impl Server {
    pub fn new() -> Server {
        let listener = TcpListener::bind(format!("{}:{}", IP, PORT)).unwrap();
        Server { listener }
    }
    pub fn listen_and_serve(&self, sender: mpsc::Sender<String>) {
        println!("listen and serve");
        let sender = Arc::new(Mutex::new(sender));
        for stream in self.listener.incoming() {
            let sx = Arc::clone(&sender);
            thread::spawn(move || {
                for message in read_connection(&mut stream.unwrap()).iter() {
                    sx.lock().unwrap().send(message.to_string()).unwrap();
                }
            });
        }
        //drop(sender);
    }
}

pub fn read_connection(stream: &mut TcpStream) -> Vec<String> {
    let mut message_vector: Vec<String> = vec![];
    let buffer = BufReader::new(stream);
    let mut buffer_split = buffer.split(b';');
    while let Some(inc_message) = buffer_split.next() {
        message_vector.push(String::from_utf8_lossy(&inc_message.unwrap()).to_string());
    }
    message_vector
}
