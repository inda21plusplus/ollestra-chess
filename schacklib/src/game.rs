use crate::board::Board;
use crate::piece::Piece;
use crate::player::Player;
use crate::square::Square;

#[derive(Debug)]
pub struct Game {
    board: Board,
    players: (Player, Player),
}

impl Game {
    pub fn new(ply1_name: String, ply2_name: String) -> Self {
        Self {
            board: Board::new(),
            players: (Player::new(ply1_name), Player::new(ply2_name)),
        }
    }
    pub fn initialize(&mut self) {
        self.board.init();
    }

    pub fn move_piece(&mut self, pos1: Square, pos2: Square) -> bool {
        // Make sure Square is not outside of the board
        if pos1.is_outside() || pos2.is_outside() {
            return false;
        }
        let piece = self.board.get(pos1);
        self.board.set(pos1, Piece::none());
        self.board.set(pos2, piece);

        return true;
    }

    pub fn get_moves(&mut self) -> [Vec<i8>; 64] {
        self.board.calculate_all_moves()
    }

    pub fn test_moves(&mut self) {
        self.board.calculate_all_moves();
    }

    //Remove whole board AKA flip the table.
    pub fn rage(&mut self) {
        self.board._reset();
    }
}
