use crate::fen_parser::FenParser;
use crate::move_offset::MoveOffset;
use crate::piece::Piece;
use crate::square::Square;
use arr_macro::arr;
use std::cmp;
use std::vec;

#[derive(Debug)]
pub(crate) struct Board {
    pub squares: [i8; 64],
    pre_calc_edge: [Vec<i8>; 64],
    white_attack_map: u64,
    black_attack_map: u64,
    king_moved: [bool; 2],
}

//Public usable functions
impl Board {
    pub fn new() -> Self {
        Self {
            squares: [0; 64],
            pre_calc_edge: arr![Vec::new(); 64],
            white_attack_map: 0,
            black_attack_map: 0,
            king_moved: [false, false],
        }
    }

    pub fn init(&mut self) {
        self.calulate_edges();

        // Starting pieces
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
        self.pre_calc_edge = arr![Vec::new(); 64];
        self.white_attack_map = 0;
        self.black_attack_map = 0;
        self.king_moved = [false, false];
    }
}

//Movement Logic - Private functions
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

    fn calculate_all_moves(&mut self) {
        self.white_attack_map = 0;
        self.black_attack_map = 0;
        for i in 0..64 {
            let sq: Square = Square::from_i8(i);
            self.calculate_moves(sq);
        }
    }

    fn add_danger(&mut self, sq: i8, c: i8) {
        if c == Piece::white() {
            if !Self::get_bit(self.white_attack_map, sq) {
                self.white_attack_map += 2i8.pow(sq as u32) as u64;
            }
        } else {
            if !Self::get_bit(self.black_attack_map, sq) {
                self.black_attack_map += 2i8.pow(sq as u32) as u64;
            }
        }
    }

    fn get_bit(num: u64, i: i8) -> bool {
        if i < 64 {
            return num & (1 << i) != 0;
        }

        false
    }

    fn straight_moves(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);
        let opponent_color = Piece::get_opponent_color(piece);

        let mut counter: usize = 0;
        for dir in MoveOffset::STRAIGHT {
            for pos in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                let sq = square.to_i8() + dir * (pos + 1);
                let p_sq = self.squares[sq as usize];

                self.add_danger(sq, color);
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

    fn askew_moves(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);
        let opponent_color = Piece::get_opponent_color(piece);

        let mut counter: usize = 4; // Offset to start at right position for Askew function
        for dir in MoveOffset::ASKEW {
            for pos in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                let sq = square.to_i8() + dir * (pos + 1);
                let p_sq = self.squares[sq as usize];

                self.add_danger(sq, color);
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

    fn knight_moves(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);

        for pos in MoveOffset::KNIGHT {
            let sq = square.to_i8() + pos;
            if Square::from_i8(sq).is_outside() {
                continue;
            }
            let p_sq = self.squares[sq as usize];

            self.add_danger(sq, color);
            if Piece::cmp_color(p_sq, color) {
                //Team piece -> Can't move further
                continue;
            }

            moves.push(sq);
        }

        moves
    }

    fn king_moves(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);
        let opponent_color = Piece::get_opponent_color(piece);

        for pos in MoveOffset::STRAIGHT.iter().chain(MoveOffset::ASKEW.iter()) {
            let sq = square.to_i8() + pos;
            if Square::from_i8(sq).is_outside() {
                continue;
            }
            let p_sq = self.squares[sq as usize];

            self.add_danger(sq, color);
            if Piece::cmp_color(p_sq, color) {
                //Team piece -> Can't move further
                continue;
            }

            moves.push(sq);
        }

        moves
    }

    fn pawn(&mut self, square: Square) -> Vec<i8> {
        unimplemented!()
    }

    fn rook(&mut self, square: Square) -> Vec<i8> {
        self.straight_moves(square)
    }

    fn knight(&mut self, square: Square) -> Vec<i8> {
        self.knight_moves(square)
    }

    fn bishop(&mut self, square: Square) -> Vec<i8> {
        self.askew_moves(square)
    }

    fn queen(&mut self, square: Square) -> Vec<i8> {
        let mut moves = self.straight_moves(square);
        moves.append(&mut self.askew_moves(square));

        moves
    }

    fn king(&mut self, square: Square) -> Vec<i8> {
        self.king_moves(square)
    }
}
