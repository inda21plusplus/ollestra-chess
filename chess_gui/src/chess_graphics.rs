use chess;
use chess::square::Square;
use ggez;
use ggez::event::{self, EventHandler, MouseButton};
use ggez::graphics;
use ggez::graphics::{Image, Mesh};
use ggez::{Context, ContextBuilder, GameResult};

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

pub struct ChessGame {
    game: chess::game::Game,
    graphics_board: [[graphics::Color; 8]; 8],
    graphics_pieces: [[String; 8]; 8],
    selection: Selection,
    update: bool,
    window: bool,
}

impl ChessGame {
    pub fn new(context: &mut Context) -> ChessGame {
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
        let mut graphics_pieces_str: [[&str; 8]; 8] = [
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
        ChessGame {
            game,
            graphics_board,
            graphics_pieces,
            selection,
            update,
            window,
        }
    }
}

impl EventHandler<ggez::GameError> for ChessGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // Update code here...
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
            println!("checkpoint 1");
            let selection_mesh =
                draw_selection_square(ctx, self, self.selection.x, self.selection.y);
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
            if verify_move(self, x as i8, y as i8) {
                move_piece_graphics(self, (self.selection.x, self.selection.y), (x, y));
                //self.draw(ctx);
            }
            self.update = true;
            self.selection.selection = false;
        }
    }
}
fn verify_move(game: &mut ChessGame, x: i8, y: i8) -> bool {
    let from = (game.selection.x) + (8 * game.selection.y);
    let to = (x) + (8 * y);

    println!("from: {}, to: {}", from, to);

    let moves = game.game.board.calculate_all_moves();
    if moves[from].contains(&to) {
        game.game
            .move_piece(Square::from_i8(from as i8), Square::from_i8(to as i8));
        return true;
    } else {
        false
    }
}

fn move_piece_graphics(game: &mut ChessGame, from: (usize, usize), to: (usize, usize)) {
    let none = "None".to_owned();
    game.graphics_pieces[to.1][to.0] = game.graphics_pieces[from.1][from.0].clone();
    game.graphics_pieces[from.1][from.0] = none;
    game.update = true;
}

fn draw_selection_square(ctx: &mut Context, game: &mut ChessGame, x: usize, y: usize) -> Mesh {
    println!("checkpoint 2");
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
