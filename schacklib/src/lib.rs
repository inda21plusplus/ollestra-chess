#[allow(dead_code)]
mod board;
mod fen_parser;
mod game;
mod move_offset;
mod piece;
mod player;
mod square;

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
    }
}
