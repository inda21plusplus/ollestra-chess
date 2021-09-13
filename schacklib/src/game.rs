use crate::errors::ChessError;
use crate::{position::Position, team::TeamColor};
use crate::piece::*;
use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use std::vec;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Game {
    pub players: (Player, Player),
    board: (u8, u8),
    pieces: Vec<Piece>,
    time_passed: u64,
    is_white_turn: bool,
}

#[derive(Debug, Clone)]
pub struct Player {
    pub name: String,
    is_white: bool,
    moves: Vec<Move>,
    points: u8,
}
#[derive(Debug, Clone, Copy)]
pub struct Move {
    from: Position,
    to: Position,
}

impl Game {
    pub fn new(p1_name: String, p2_name: String) -> Game {
        Self {
            board: (8, 8),
            players: (
                Player {
                    name: p1_name,
                    is_white: true,
                    moves: vec![],
                    points: 0,
                },
                Player {
                    name: p2_name,
                    is_white: false,
                    moves: vec![],
                    points: 0,
                },
            ),
            pieces: vec!(),
            time_passed: 0,
            is_white_turn: true,
        }
    }

    pub fn init(&mut self) {
        self.pieces = self.init_pieces();
    }

    pub fn get_piece_at(&self, pos: Position) -> Result<&Piece, ChessError> {
        for piece in &self.pieces {
            if pos == *piece.get_position() {
                return Ok(piece);
            }
        }
        return Err(ChessError::NullPosition {x: pos.rank, y: pos.file });
    }

    pub fn get_piece_at_mut(&mut self, pos: Position) -> Result<&mut Piece, ChessError> {
        for piece in &mut self.pieces {
            if pos == *piece.get_position() {
                return Ok(piece);
            }
        }
        return Err(ChessError::NullPosition {x: pos.rank, y: pos.file });
    }

    pub fn make_move_p2p(&mut self, from: Position, to: Position) -> bool {
        let piece: &Piece = match self.get_piece_at_mut(from) {
            Ok(p) => p,
            Err(_) => return false
        };

        let p_moves: Vec<(Position, bool)> = piece.calculate_possible_moves(&self.pieces);

        let mut move_instructions: (bool, bool) = (false, false);
        for p_move in p_moves {
            if p_move.0 == to {
                move_instructions.0 = true;
                move_instructions.1 = p_move.1;
                break;
            }
        }
        
        if move_instructions.0 {        // If move is possible
            if move_instructions.1 {    // If move should take another piece
                let index = self.pieces.iter().position(|r| *r.get_position() == to).unwrap();
                piece.set_position(to);
                self.pieces.remove(index);
            } else {
                piece.set_position(to);
            }
            return true;
        }
        return false;
    }

    fn remove_piece(&mut self, index: usize) {
        &self.pieces.remove(index);
    }

    fn init_pieces(&self) -> Vec<Piece> {
        let mut pieces: Vec<Piece> = vec![];
        let mut counter = 0;

        // Create Pawns
        for i in 0..2 {
            let file: u8 = (i * 7) + 1 - i;
            for rank in 0..8 {
                pieces.push(Piece::new(
                    counter,
                    PieceType::PAWN,
                    if i == 0 {
                        TeamColor::WHITE
                    } else {
                        TeamColor::BLACK
                    },
                    Position { rank, file }
                ));
                counter = counter + 1;
            }
        }
        pieces
    }
}
