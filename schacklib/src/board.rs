use crate::fen_parser::FenParser;
use crate::move_offset::MoveOffset;
use crate::piece::Piece;
use crate::square;
use crate::square::Square;
use arr_macro::arr;
use std::cmp;
use std::fmt::format;
use std::vec;

#[derive(Debug)]
pub(crate) struct Board {
    pub squares: [i8; 64],
    pre_calc_edge: [Vec<i8>; 64],
    knight_moves: [Vec<i8>; 64],
    ray_map: [Vec<(i8, bool)>; 64],
    white_attack_map: u64,
    black_attack_map: u64,
    king_in_check: [bool; 2],
}

//Public usable functions
impl Board {
    pub fn new() -> Self {
        Self {
            squares: [0; 64],
            pre_calc_edge: arr![Vec::new(); 64],
            knight_moves: arr![Vec::new(); 64],
            ray_map: arr![Vec::new(); 64],
            white_attack_map: 0,
            black_attack_map: 0,
            king_in_check: [false, false],
        }
    }

    pub fn init(&mut self) {
        // Starting pieces
        //self.squares = FenParser::decode(FenParser::STARTING_POSITION.to_string());

        self.squares[3] = Piece::king(true);
        self.squares[11] = Piece::rook(true);

        self.squares[35] = Piece::rook(false);
        self.squares[53] = Piece::king(false);

        // Calculate closest edge
        self.calulate_edges();
    }

    pub fn set(&mut self, square: Square, piece_value: i8) {
        self.squares[square.to_i8() as usize] = piece_value;
    }

    pub fn get(&self, square: Square) -> i8 {
        self.squares[square.to_i8() as usize]
    }

    pub fn calculate_moves(&mut self, square: Square) -> Vec<i8> {
        let piece = self.squares[square.to_i8() as usize];

        match piece & Piece::PIECE_MASK {
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
        self.knight_moves = arr![Vec::new(); 64];
        self.ray_map = arr![Vec::new(); 64];
        self.white_attack_map = 0;
        self.black_attack_map = 0;
        self.king_in_check = [false, false];
    }
}

//Movement Logic - Private functions
impl Board {
    fn calulate_edges(&mut self) {
        for file in 0..8 {
            for rank in 0..8 {
                let index = rank * 8 + file;

                let up: i8 = 7 - rank;
                let down: i8 = rank;
                let right: i8 = 7 - file;
                let left: i8 = file;
                self.pre_calc_edge[index as usize] = vec![
                    up,
                    down,
                    left,
                    right,
                    cmp::min(up, left),
                    cmp::min(down, right),
                    cmp::min(up, right),
                    cmp::min(down, left),
                ];

                //Knight moves
                let mut knight_moves: Vec<i8> = vec![];
                for knight_jmp in MoveOffset::KNIGHT {
                    let target_sq = index + knight_jmp;
                    if target_sq >= 0 && target_sq < 64 {
                        let y: i8 = target_sq / 8;
                        let x: i8 = target_sq - y * 8;

                        //https://www.chessprogramming.org/Knight-Distance - Make sure Knight can't wrap around board
                        let max_dst: i8 = cmp::max(i8::abs(file - x), i8::abs(rank - y));
                        if max_dst == 2 {
                            knight_moves.push(target_sq);
                        }
                    }
                }
                self.knight_moves[index as usize] = knight_moves;
            }
        }
    }

    pub fn calculate_all_moves(&mut self) -> [Vec<i8>; 64] {
        self.white_attack_map = 0;
        self.black_attack_map = 0;
        self.ray_map = arr![Vec::new(); 64];

        let mut out: [Vec<i8>; 64] = arr![Vec::new(); 64];
        for i in 0..64 {
            let sq: Square = Square::from_i8(i);
            if self.get(sq) != Piece::none() {
                let moves = self.calculate_moves(sq);
                out[i as usize] = moves;
            }
        }
        out
    }

    fn add_danger(&mut self, sq: i8, c: i8) {
        if c == Piece::white() {
            if !Self::get_bit(self.white_attack_map, sq) {
                if self.get(Square::from_i8(sq)) == Piece::king(false) {
                    self.king_in_check[1] = true;
                }
                self.white_attack_map |= 1u64 << sq;
            }
        } else {
            if !Self::get_bit(self.black_attack_map, sq) {
                if self.get(Square::from_i8(sq)) == Piece::king(false) {
                    self.king_in_check[0] = true;
                }
                self.black_attack_map |= 1u64 << sq;
            }
        }
    }

    fn remove_danger(&mut self, sq: i8, c: i8) {
        if c == Piece::white() {
            if Self::get_bit(self.white_attack_map, sq) {
                self.white_attack_map |= 1u64 >> sq;
            }
        } else {
            if Self::get_bit(self.black_attack_map, sq) {
                self.black_attack_map |= 1u64 >> sq;
            }
        }
    }

    fn in_danger(&mut self, sq: i8, c: i8) -> bool {
        if c == Piece::white() {
            Self::get_bit(self.white_attack_map, sq)
        } else {
            Self::get_bit(self.black_attack_map, sq)
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
        let mut p_target: (i8, bool, i8) = (0, false, 0); //Piece, is_white, square
        for dir in MoveOffset::STRAIGHT {
            for pos in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                let sq = square.to_i8() + dir * (pos + 1);
                let p_sq = self.squares[sq as usize];

                self.add_danger(sq, color);
                if Piece::cmp_color(p_sq, color) {
                    //Team piece -> Can't move further
                    p_target = (p_sq, color == Piece::white(), sq);
                    break;
                }

                moves.push(sq);
                if Piece::cmp_color(p_sq, opponent_color) {
                    //Opponent piece -> Can move to opponent piece
                    p_target = (p_sq, color == Piece::white(), sq);
                    break;
                }
            }
            // Rays
            if p_target.0 > 0 {
                for ray in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                    let sq = square.to_i8() + dir * (ray + 1);
                    let p_sq = self.squares[sq as usize];

                    if p_sq == Piece::king(!p_target.1) {
                        p_target.0 |= 1i8 << 6;
                        self.set(Square::from_i8(sq), p_target.0);
                    }
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

        let mut counter: usize = 4; // Offset to start at correct position for Askew offsets
        let mut p_target: (i8, bool, i8) = (0, false, 0); //Piece, is_white, square
        for dir in MoveOffset::ASKEW {
            for pos in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                let sq = square.to_i8() + dir * (pos + 1);
                let p_sq = self.squares[sq as usize];

                self.add_danger(sq, color);
                if Piece::cmp_color(p_sq, color) {
                    //Team piece -> Can't move further
                    p_target = (p_sq, color == Piece::white(), sq);
                    break;
                }

                moves.push(sq);
                if Piece::cmp_color(p_sq, opponent_color) {
                    //Opponent piece -> Can move to opponent piece
                    p_target = (p_sq, color == Piece::white(), sq);
                    break;
                }
            }
            // Rays
            if p_target.0 > 0 {
                for ray in 0..self.pre_calc_edge[square.to_i8() as usize][counter] {
                    let sq = square.to_i8() + dir * (ray + 1);
                    let p_sq = self.squares[sq as usize];

                    if p_sq == Piece::king(!p_target.1) {
                        p_target.0 |= 1i8 << 6;
                        self.set(Square::from_i8(sq), p_target.0);
                    }
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
        let list = self.knight_moves[square.to_i8() as usize].clone();

        for pos in list {
            let p_sq = self.squares[pos as usize];

            self.add_danger(pos, color);
            if Piece::cmp_color(p_sq, color) {
                //Team piece -> Can't move further
                continue;
            }

            moves.push(pos);
        }

        moves
    }

    fn king_moves(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);

        for pos in MoveOffset::STRAIGHT.iter().chain(MoveOffset::ASKEW.iter()) {
            let sq = square.to_i8() + pos;
            if sq > 63 || sq < 0 {
                continue;
            }

            if self.in_danger(sq, color) {
                // Square in attack position
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

    fn pawn_moves(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];
        let piece = self.get(square);
        let color = Piece::get_color(piece);
        let offset = if color == Piece::white() {
            MoveOffset::PAWN_WHITE
        } else {
            MoveOffset::PAWN_BLACK
        };
        let start_square = if color == Piece::white() { 1 } else { 6 };

        let mut is_capture: bool = false;
        let mut counter: i8 = 0;
        for pos in offset {
            let sq: i8 = square.to_i8() + pos;
            if sq < 0 && sq > 63 {
                continue;
            }

            if !is_capture {
                if self.get(Square::from_i8(sq)) > 0 {
                    continue;
                }

                moves.push(sq);

                if start_square == square.rank && self.get(Square::from_i8(sq + pos)) == 0 {
                    moves.push(sq + pos);
                }
            } else {
                if counter == 2 && square.file < 1 {
                    continue;
                } else if counter == 3 && square.file > 6 {
                    continue;
                }

                if self.get(Square::from_i8(sq)) == 0 {
                    continue;
                }
                moves.push(sq);
            }
            is_capture = true;
            counter += 1;
        }

        moves
    }

    fn castle(&mut self, square: Square) -> Vec<i8> {
        let mut moves: Vec<i8> = vec![];

        moves
    }

    fn pawn(&mut self, square: Square) -> Vec<i8> {
        self.pawn_moves(square)
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
        let mut moves = self.king_moves(square);
        moves.append(&mut self.castle(square));

        moves
    }
}
