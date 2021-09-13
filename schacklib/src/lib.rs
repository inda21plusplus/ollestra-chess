#[allow(dead_code)]
pub mod errors;
pub mod game;
pub mod piece;
mod team;
mod movement_behavior;
mod position;

#[cfg(test)]
mod tests {
    use crate::game::Game;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn game_init() {
        let mut game = Game::new("Player 1".to_string(), "Player 2".to_string());
        game.init();

        dbg!(game);
    }
}
