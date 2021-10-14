use crate::fen_handler;
use chess;
use chess::square::Square;
use ggez;
use ggez::event::{EventHandler, MouseButton};
use ggez::graphics;
use ggez::graphics::Mesh;
use ggez::{Context, ContextBuilder, GameResult};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{self, channel, Receiver, Sender};
use std::thread;
//use crate::network::Server;

const board_letters: [&str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];

struct Selection {
    selection: bool,
    x: usize,
    y: usize,
}
impl Selection {
    fn new() -> Selection {
        let selection = false;
        let x = 0;
        let y = 0;
        Selection { selection, x, y }
    }
}

pub struct ChessGame_multi {
    game: chess::game::Game,
    graphics_board: [[graphics::Color; 8]; 8],
    graphics_pieces: [[String; 8]; 8],
    selection: Selection,
    update: bool,
    window: bool,
    turn_white: bool,
    receiver: Receiver<String>,
    response: Sender<String>,
    last_move: String,
}

impl ChessGame_multi {
    pub fn new(
        _context: &mut Context,
        receiver: Receiver<String>,
        response: Sender<String>,
    ) -> ChessGame_multi {
        let mut game = chess::game::Game::new("player1".to_string(), "player2".to_string());
        game.initialize();
        let mut graphics_board = [[graphics::Color::WHITE; 8]; 8];
        for i in 0..8 {
            for j in 0..8 {
                if (i % 2 == 0 && j % 2 == 0) || (i % 2 != 0 && j % 2 != 0) {
                    graphics_board[i][j] = graphics::Color::WHITE;
                } else {
                    graphics_board[i][j] = graphics::Color::BLUE;
                }
            }
        }
        let mut graphics_pieces: [[String; 8]; 8] = Default::default();
        let graphics_pieces_str: [[&str; 8]; 8] = [
            ["rd", "nd", "bd", "qd", "kd", "bd", "nd", "rd"],
            ["pd", "pd", "pd", "pd", "pd", "pd", "pd", "pd"],
            [
                "None", "None", "None", "None", "None", "None", "None", "None",
            ],
            [
                "None", "None", "None", "None", "None", "None", "None", "None",
            ],
            [
                "None", "None", "None", "None", "None", "None", "None", "None",
            ],
            [
                "None", "None", "None", "None", "None", "None", "None", "None",
            ],
            ["pl", "pl", "pl", "pl", "pl", "pl", "pl", "pl"],
            ["rl", "nl", "bl", "ql", "kl", "bl", "nl", "rl"],
        ];
        for i in 0..8 {
            for j in 0..8 {
                graphics_pieces[i][j] = graphics_pieces_str[i][j].to_owned();
            }
        }
        let selection = Selection::new();
        let update = true;
        let window = false;
        let turn_white = true;
        let last_move = String::new();
        ChessGame_multi {
            game,
            graphics_board,
            graphics_pieces,
            selection,
            update,
            window,
            turn_white,
            receiver,
            response,
            last_move,
        }
    }
}

impl EventHandler<ggez::GameError> for ChessGame_multi {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        //Update code here...
        if !self.turn_white && !self.update {
            //println!("checkpoint 1");

            //self.listener.listener.set_nonblocking(true).expect("error");
            //self.listener.listen_and_serve(sender);

            let input = self.receiver.try_recv();
            if input.is_err() {
                return Ok(());
            }
            let input = input.unwrap();
            if input.starts_with("move") {
                let ((from, i), (to, j)) = verify_input(&input).unwrap();

                let pos1 = Square::from_i8((from as i8 * 8) + i as i8);
                let pos2 = Square::from_i8((to as i8 * 8) + j as i8);

                self.game.move_piece(pos1, pos2);

                let none = "None".to_owned();
                self.graphics_pieces[to][j] = self.graphics_pieces[from][i].clone();
                self.graphics_pieces[from][i] = none;
                self.update = true;
                self.turn_white = true;
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.update == false {
            return Ok(());
        }
        graphics::clear(ctx, graphics::Color::from_rgb(100, 0, 100));
        let window = graphics::Rect::new(0f32, 0f32, 1600f32, 1600f32);
        if !self.window {
            let width = 100f32;
            graphics::clear(ctx, graphics::Color::from_rgb(100, 0, 100));
            graphics::set_window_title(ctx, "Daniel's Chess");
            //graphics::set_fullscreen(ctx, ggez::conf::FullscreenType::Desktop);

            //graphics::set_fullscreen(ctx, ggez::conf::FullscreenType::True);
            //graphics::set_fullscreen(ctx, ggez::conf::FullscreenType::True);
            //graphics::set_screen_coordinates(ctx, window);
        }

        // Draw code here...
        for i in 0..8 {
            for j in 0..8 {
                let square =
                    graphics::Rect::new((i) as f32 * 100.0, (j) as f32 * 100.0, 100.0, 100.0);
                let square: graphics::Mesh = graphics::Mesh::new_rectangle(
                    ctx,
                    graphics::DrawMode::fill(),
                    square,
                    self.graphics_board[i][j],
                )?;
                graphics::draw(ctx, &square, graphics::DrawParam::default());
            }
        }
        for i in 0..8 {
            for j in 0..8 {
                let name = self.graphics_pieces[i][j].clone();
                if name != "None" {
                    let filename = ["/".to_owned(), name.to_owned(), ".png".to_owned()].join("");
                    let piece_image = graphics::Image::new(ctx, filename);
                    let x = 100f32 * (j) as f32;
                    let y = 100f32 * (i) as f32;
                    let params = graphics::DrawParam::new().dest([x, y]).scale([2.0, 2.0]);

                    graphics::draw(ctx, &piece_image.unwrap(), params)?;
                }
            }
        }

        // let testpiece = graphics::Image::new(ctx, "bd.png") as Option<graphics::ImageGeneric>;
        //name
        // let indices = &[1];
        // let piecemesh = graphics::Mesh::from_raw(ctx, &[], indices, testpiece)?;
        // graphics::draw(ctx, &piecemesh, graphics::DrawParam::default());
        if self.selection.selection {
            let selection_mesh = draw_selection_square(ctx, self.selection.x, self.selection.y);
            graphics::draw(ctx, &selection_mesh, graphics::DrawParam::default());
        }
        self.update = false;
        //graphics::set_screen_coordinates(ctx, window);
        graphics::present(ctx)
    }
    fn mouse_button_down_event(&mut self, ctx: &mut Context, btn: MouseButton, x: f32, y: f32) {
        //let letters = ["A", "B", "C", "D", "E", "F", "G", "H"];

        let x = ((x as i32) / 100i32) as usize;
        let y = ((y as i32) / 100i32) as usize;
        if !self.selection.selection {
            let piece = self.graphics_pieces[y as usize][x as usize].clone();

            self.selection.selection = true;
            self.selection.x = x as usize;
            self.selection.y = y as usize;
            self.update = true;
        } else {
            if self.turn_white {
                if verify_move(self, x as i8, y as i8) {
                    move_piece_graphics(self, (self.selection.x, self.selection.y), (x, y));
                    //self.draw(ctx);
                    // let response_string = format!(
                    //     "move::{}{}{}{}-;",
                    //     board_letters[self.selection.x],
                    //     self.selection.y + 1,
                    //     board_letters[x],
                    //     y + 1
                    // );
                    // println!("message sent {}", response_string);
                    let response_string =
                        fen_handler::to_fen(&self.graphics_pieces, "w".to_string());

                    println!("sent fen string: {}", response_string);
                    let send = self.response.send(response_string);

                    if send.is_err() {
                        panic!();
                    }

                    self.turn_white = false;
                }
            }
            self.update = true;
            self.selection.selection = false;
        }
    }
}
fn verify_move(game: &mut ChessGame_multi, x: i8, y: i8) -> bool {
    let from = (game.selection.x) + (8 * game.selection.y);
    let to = (x) + (8 * y);

    let moves = game.game.board.calculate_all_moves();
    if moves[from].contains(&to) {
        game.game
            .move_piece(Square::from_i8(from as i8), Square::from_i8(to as i8));
        return true;
    } else {
        false
    }
}

fn move_piece_graphics(game: &mut ChessGame_multi, from: (usize, usize), to: (usize, usize)) {
    let none = "None".to_owned();
    game.graphics_pieces[to.1][to.0] = game.graphics_pieces[from.1][from.0].clone();
    game.graphics_pieces[from.1][from.0] = none;
    game.update = true;
}

fn draw_selection_square(ctx: &mut Context, x: usize, y: usize) -> Mesh {
    let square = graphics::Rect::new(x as f32 * 100.0, y as f32 * 100.0, 100.0, 100.0);
    let square_mesh = graphics::Mesh::new_rectangle(
        ctx,
        graphics::DrawMode::stroke(10.0),
        square,
        graphics::Color::YELLOW,
    )
    .unwrap();
    square_mesh
}

fn verify_input(input: &String) -> Result<((usize, usize), (usize, usize)), String> {
    if input.len() < 4 {
        return Err("false input".to_owned());
    }

    //Convertera user input till index
    let table = ["a", "b", "c", "d", "e", "f", "g", "h"];
    println!("{},{}", &&input[5..6], &&input[7..8]);
    if !table.contains(&&input[5..6]) || !table.contains(&&input[7..8]) {
        return Err("Invalid Input".to_owned());
    }

    //parse the digits and convert them to usize
    let parsed = &input[6..7];
    let from: usize = match parsed.parse::<usize>() {
        Ok(value) => value.to_owned() - 1,
        Err(error) => {
            return Err(error.to_string());
        }
    };
    //let from = from.to_owned();
    //let from = from - 1;
    let parsed = &input[8..9];
    let to: usize = match parsed.parse::<usize>() {
        Ok(value) => value.to_owned() - 1,
        Err(error) => {
            println!("{}", error);
            return Err("Internal Error".to_owned());
        }
    };
    //iterate digits from input
    let i = table.iter().position(|&s| s == &input[5..6]).unwrap();
    let j = table.iter().position(|&s| s == &input[7..8]).unwrap();
    return Ok(((from, i), (to, j)));
}
