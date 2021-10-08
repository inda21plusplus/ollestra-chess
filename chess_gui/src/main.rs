use ggez;
use ggez::event;
use ggez::{ContextBuilder, GameResult};
use std::env;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;
//use glam::*;
const IP: &str = "127.0.0.1";
const PORT: &str = "1337";

mod chess_graphics;
use chess_graphics::ChessGame;

mod chess_graphics_multi;
mod network;

struct GameSettings {
    mode: &'static str,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    if args.len() <= 3 {
        args.push("temp".to_owned());
        if args.len() <= 2 {
            args.push("temp".to_owned());
        }
    }
    println!("{:?}, length: {}", args, args.len());

    let settings: GameSettings = match (args[1].as_str(), args[2].as_str()) {
        ("--multi", "--client") => GameSettings { mode: "client" },
        ("--multi", _) => GameSettings { mode: "server" },
        (_, _) => GameSettings { mode: "single" },
    };

    //define application window
    let resolution = ggez::conf::WindowMode::default().dimensions(800., 800.);
    let (mut context, event_loop) = ContextBuilder::new("chess", "Daniel Tottie")
        .window_mode(resolution)
        .resources_dir_name("../../chess_gui/src/resources")
        .build()
        .expect("error");

    //launch GUI in desired mode
    if settings.mode == "server" {
        let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let (response, responder): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();

        thread::spawn(move || {
            let listener = TcpListener::bind(format!("{}:{}", IP, PORT)).unwrap();
            let sender = Arc::new(Mutex::new(sender));
            for stream in listener.incoming() {
                println!("check");
                let mut stream = stream.unwrap();
                let mut stream_clone = Arc::new(Mutex::new(&stream));

                // for message in network::read_connection(&mut stream) {
                //     sx.lock().unwrap().send(message.to_string()).unwrap();
                // }
                let buffer = BufReader::new(&stream);
                let mut buffer_split = buffer.split(b';');
                println!("checkpoint1");
                let pmove = responder.recv();
                while let Some(inc_message) = buffer_split.next() {
                    println!("checkpoint 2");
                    //let pmove = responder.recv();
                    let inc_message = String::from_utf8_lossy(&inc_message.unwrap()).to_string();
                    let inc_message = inc_message.replace('\n', "");

                    if !inc_message.starts_with("init") {
                        let sx = Arc::clone(&sender);
                        let _transmit = sx.lock().unwrap().send(inc_message);
                        drop(sx);
                        continue;
                        let pmove = responder.recv();
                        let pmove = pmove.unwrap();
                        println!("the following was sent to the client: {}", pmove);
                        stream_clone.lock().unwrap().write(pmove.as_bytes());
                    }

                }
            }

            //CONTINUE herebuf
            //MAKE CODE SEND BACK A STATUS REPORT THAT A MOVE HAS BEEN MADE
            // match responder.try_recv() {
            //     Ok(_) => {
            //         println!("received.");
            //         for stream in listener.incoming() {
            //             let mut stream = stream.unwrap();
            //             stream.flush();
            //             stream.write(b"update");
            //         }
            //     }
            //     Err(_) => println!("did not receive anything"),
            //     _ => print!("checkpoint 1"),
            // }
        });

        let chess_game =
            chess_graphics_multi::ChessGame_multi::new(&mut context, receiver, response);

        event::run(context, event_loop, chess_game);
    }
    if settings.mode == "single" {
        let chess_game = ChessGame::new(&mut context);
        event::run(context, event_loop, chess_game)
    }
    if settings.mode == "client" {
        let mut connection = TcpStream::connect("127.0.0.1:1337").unwrap();
        connection.write(b"init:;").unwrap();

        loop {
            let mut input = String::new();
            println!("input:");
            io::stdin().read_line(&mut input).expect("woops");

            connection.write_all(input.as_bytes()).unwrap();
            //connection.flush().unwrap();
        }
    }
}
