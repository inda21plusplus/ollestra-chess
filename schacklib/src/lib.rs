#[allow(dead_code)]
pub mod board;
pub mod fen_parser;
pub mod game;
mod move_offset;
mod piece;
mod player;
pub mod square;

#[cfg(test)]
mod tests {
    use crate::{game::Game, square::Square};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn create_piece() {
        let mut game: Game = Game::new("Player 1".to_string(), "Player 2".to_string());
        game.initialize();

        game.test_moves();
    }
}
