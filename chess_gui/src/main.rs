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

mod fen_handler;

mod chess_graphics_multi;
//mod network;

mod chess_graphics_multi_client;

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
                //println!("check");
                let mut stream = stream.unwrap();
                let mut stream_clone = Arc::new(Mutex::new(&stream));
                // for message in network::read_connection(&mut stream) {
                //     sx.lock().unwrap().send(message.to_string()).unwrap();
                // }
                let buffer = BufReader::new(&stream);
                let mut buffer_split = buffer.split(b';');
                //println!("checkpoint 1");
                let pmove = responder.recv();
                stream_clone
                    .lock()
                    .unwrap()
                    .write(pmove.unwrap().as_bytes())
                    .expect("error in sending");
                stream_clone.lock().unwrap().flush().unwrap();
                while let Some(inc_message) = buffer_split.next() {
                    let inc_message = inc_message.unwrap();
                    println!("message received in server: {:?}", inc_message);
                    //println!("checkpoint 2");
                    //let pmove = responder.recv();
                    let inc_message = String::from_utf8_lossy(&inc_message).to_string();
                    let inc_message = inc_message.replace('\n', "");

                    if !inc_message.starts_with("init") {
                        //println!("checkpoint 3");
                        let sx = Arc::clone(&sender);
                        let _transmit = sx.lock().unwrap().send(inc_message);
                        drop(sx);
                        let pmove = responder.recv();
                        println!("received from responder");
                        let pmove = pmove.unwrap();
                        println!("the following was sent to the client: {}", pmove);
                        stream_clone
                            .lock()
                            .unwrap()
                            .write_all(pmove.as_bytes())
                            .expect("error in sending");
                        stream_clone.lock().unwrap().flush().unwrap();
                    }

                    stream_clone.lock().unwrap().flush().unwrap();
                }

                drop(stream_clone);
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
        let fen_string =
            "board:rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2".to_owned();
        let board = fen_handler::fen_parse(fen_string);

        let chess_game = ChessGame::new(&mut context);
    }
    if settings.mode == "client" {
        let (response, responder): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        let (sender, receiver): (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel();
        // loop {
        //     let mut buf = BufReader::new(&mut connection);
        //     let mut bufsplit = buf.split(b';');
        //     while let Some(messy) = bufsplit.next() {
        //         let messy = String::from_utf8_lossy(&messy.unwrap()).to_string();
        //         let messy = messy.replace('\n', "");
        //         println!("received: {:?}", messy);
        //         if messy.starts_with("move") {
        //             break;
        //         }
        //     }
        //
        //     let mut input = String::new();
        //     println!("input:");
        //     io::stdin().read_line(&mut input).expect("woops");
        //
        //     connection.write_all(input.as_bytes()).unwrap();
        //     //connection.flush().unwrap();
        // }
        thread::spawn(move || {
            let mut connection = TcpStream::connect(format!("{}:{}", IP, PORT)).unwrap();
            let mut connection_clone = Arc::new(Mutex::new(&connection));
            println!("connected");
            let sender = Arc::new(Mutex::new(sender));
            connection_clone
                .lock()
                .unwrap()
                .write_all(b"init:;")
                .unwrap();
            println!("send message to stream");
            let buf = BufReader::new(&connection);
            let mut bufsplit = buf.split(b';');
            while let Some(message) = bufsplit.next() {
                let sx = Arc::clone(&sender);

                let message = String::from_utf8_lossy(&message.unwrap()).to_string();
                let message = message.replace('\n', "");
                println!("received: {}", message);
                if message.starts_with("board:") {
                    sx.lock().unwrap().send(message).expect("could not send");
                }
                let client_move = responder.recv();
                if client_move.is_err() {
                    continue;
                }
                let client_move = client_move.unwrap();
                connection_clone
                    .lock()
                    .unwrap()
                    .write_all(client_move.as_bytes())
                    .unwrap();
            }
        });

        let chess_game =
            chess_graphics_multi_client::ChessGame_multi::new(&mut context, receiver, response);
        event::run(context, event_loop, chess_game);
    }
}
