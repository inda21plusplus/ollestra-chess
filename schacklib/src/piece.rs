use crate::position::Position;
use crate::movement_behavior::MovementBehavior;
use crate::team::TeamColor;

#[derive(Debug, Clone)]
pub struct Piece {
    pub id: u16,
    pub rank: PieceType,
    pub team: TeamColor,
    pub is_first_move: bool,
    position: Position,
    is_pinned: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    KING,
    QUEEN,
    ROOK,
    BISHOP,
    KNIGHT,
    PAWN,
}

impl Piece {
    pub fn get_position(&self) -> &Position {
        &self.position
    }

    pub fn set_position(&mut self, pos: Position) {
        self.position = pos;
    }

    pub fn get_pinned(&self) -> bool {
        self.is_pinned
    }

    pub fn new(id: u16, rank: PieceType, team: TeamColor, position: Position) -> Piece {
        Self {
            id,
            rank,
            team,
            position,
            is_first_move: true,
            is_pinned: false
        }
    }

    pub fn calculate_possible_moves(&self, pieces: &Vec<Piece>) -> Vec<(Position, bool)> {
        if self.get_pinned() {
            return vec!();
        }
        match &self.rank {
            PieceType::KING => MovementBehavior::king(&self),
            PieceType::QUEEN => MovementBehavior::queen(&self),
            PieceType::ROOK => MovementBehavior::rook(&self),
            PieceType::BISHOP => MovementBehavior::bishop(&self),
            PieceType::KNIGHT => MovementBehavior::knight(&self),
            PieceType::PAWN => MovementBehavior::pawn(&self, pieces),
        }
    }
}
