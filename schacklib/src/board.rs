use crate::fen_parser::FenParser;
use crate::move_offset::MoveOffset;
use crate::piece::Piece;
use crate::square;
use crate::square::Square;
use arr_macro::arr;
use std::cmp;
use std::vec;

#[derive(Debug)]
pub(crate) struct Board {
    pub squares: [i8; 64],
    pub pre_calc_edge: [Vec<i8>; 64],
}

//Public usable functions
impl Board {
    pub fn new() -> Self {
        Self {
            squares: [0; 64],
            pre_calc_edge: arr![Vec::new(); 64],
        }
    }

    pub fn init(&mut self) {
        self.calulate_edges();

        // Starting piece
        self.squares = FenParser::decode(FenParser::STARTING_POSITION.to_string());

        dbg!(&self.squares);
    }

    pub fn set(&mut self, square: Square, piece_value: i8) {
        self.squares[square.to_i8() as usize] = piece_value;
    }

    pub fn get(&self, square: Square) -> i8 {
        self.squares[square.to_i8() as usize]
    }

    pub fn calculate_moves(&mut self, square: Square) -> Vec<i8> {
        let piece = self.squares[square.to_i8() as usize];

        match piece ^ (Piece::black() | Piece::white()) {
            1 => self.pawn(square),
            2 => self.rook(square),
            3 => self.knight(square),
            4 => self.bishop(square),
            5 => self.queen(square),
            6 => self.king(square),
            _ => vec![],
        }
    }

    pub(crate) fn _reset(&mut self) {
        self.squares = [0; 64];
    }
}

//Movement Logic
impl Board {
    fn calulate_edges(&mut self) {
        for file in 0..8 {
            for rank in 0..8 {
                let up: i8 = 7 - rank;
                let down: i8 = rank;
                let right: i8 = 7 - file;
                let left: i8 = file;

                let index = rank * 8 + file;
                self.pre_calc_edge[index as usize] = vec![
                    up,
                    down,
                    right,
                    left,
                    cmp::min(up, left),
                    cmp::min(up, right),
                    cmp::min(down, left),
                    cmp::min(down, right),
                ];
            }
        }
    }

    pub fn straight_moves(&self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);
        let opponent_color = Piece::get_opponent_color(piece);

        let mut counter: usize = 0;
        for dir in MoveOffset::STRAIGHT {
            for pos in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                let sq = square.to_i8() + dir * (pos + 1);
                let p_sq = self.squares[sq as usize];

                if Piece::cmp_color(p_sq, color) {
                    //Team piece -> Can't move further
                    break;
                }

                moves.push(sq);
                if Piece::cmp_color(piece, opponent_color) {
                    //Opponent piece -> Can move to opponent piece
                    break;
                }
            }
            counter += 1;
        }
        moves
    }

    pub fn askew_moves(&self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);
        let opponent_color = Piece::get_opponent_color(piece);

        let mut counter: usize = 4; // Offset to start at right position for Askew function
        for dir in MoveOffset::ASKEW {
            for pos in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                let sq = square.to_i8() + dir * (pos + 1);
                let p_sq = self.squares[sq as usize];

                if Piece::cmp_color(p_sq, color) {
                    //Team piece -> Can't move further
                    break;
                }

                moves.push(sq);
                if Piece::cmp_color(piece, opponent_color) {
                    //Opponent piece -> Can move to opponent piece
                    break;
                }
            }
            counter += 1;
        }
        moves
    }

    fn pawn(&self, square: Square) -> Vec<i8> {
        unimplemented!()
    }

    fn rook(&self, square: Square) -> Vec<i8> {
        self.straight_moves(square)
    }

    fn knight(&self, square: Square) -> Vec<i8> {
        unimplemented!()
    }

    fn bishop(&self, square: Square) -> Vec<i8> {
        self.askew_moves(square)
    }

    fn queen(&self, square: Square) -> Vec<i8> {
        let mut a = self.straight_moves(square);
        a.append(&mut self.askew_moves(square));

        a
    }

    fn king(&self, square: Square) -> Vec<i8> {
        unimplemented!()
    }
}
