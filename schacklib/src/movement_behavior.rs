use std::rc::Rc;

use crate::errors::ChessError;
use crate::game::{self, Game};
use crate::position::Position;
use crate::piece::*;
use crate::team::TeamColor;

#[derive(Debug, Clone)]
pub struct MovementBehavior;

impl MovementBehavior {
    pub fn king(piece: &Piece) -> Vec<(Position, bool)> {
        let moves: Vec<(Position, bool)> = vec![];
        
        moves
    }

    pub fn queen(piece: &Piece) -> Vec<(Position, bool)> {
        let moves: Vec<(Position, bool)> = vec![];

        
        moves
    }

    pub fn rook(piece: &Piece) -> Vec<(Position, bool)> {
        let moves: Vec<(Position, bool)> = vec![];

        
        moves
    }

    pub fn bishop(piece: &Piece) -> Vec<(Position, bool)> {
        let moves: Vec<(Position, bool)> = vec![];

        
        moves
    }

    pub fn knight(piece: &Piece) -> Vec<(Position, bool)> {
        let moves: Vec<(Position, bool)> = vec![];

        moves
    }

    pub fn pawn(piece: &Piece, pieces: &Vec<Piece>) -> Vec<(Position, bool)> {
        let mut moves: Vec<(Position, bool)> = vec![];
        let cur_pos = piece.get_position();

        match piece.team {
            TeamColor::WHITE => {
                moves.push((Position { file: cur_pos.file + 1, rank: cur_pos.rank }, false));
                match MovementBehavior::get_piece_at(Position { file: cur_pos.file + 1, rank: cur_pos.rank + 1}, pieces) {
                    Ok(piece) => moves.push((*piece.get_position(), true)),
                    Err(e) => println!("{}", e.to_string()),
                }

                match MovementBehavior::get_piece_at(Position { file: cur_pos.file + 1, rank: cur_pos.rank - 1}, pieces) {
                    Ok(piece) => moves.push((*piece.get_position(), true)),
                    Err(e) => println!("{}", e.to_string()),
                }

                if piece.is_first_move {
                    moves.push((Position { file: cur_pos.file + 2, rank: cur_pos.rank }, false));
                }
            },
            TeamColor::BLACK => {
                
            },
        }
        moves
    }

    pub fn get_piece_at(pos: Position, pieces: &Vec<Piece>) -> Result<&Piece, ChessError> {
        for piece in pieces {
            if pos == *piece.get_position() {
                return Ok(piece);
            }
        }
        return Err(ChessError::NullPosition {x: pos.rank, y: pos.file });
    }
}