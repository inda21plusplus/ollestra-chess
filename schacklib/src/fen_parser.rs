use std::collections::HashMap;

use crate::piece::Piece;

pub struct FenParser;

impl FenParser {
    pub const STARTING_POSITION: &'static str =
        "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

    pub fn decode(fen_string: String) -> [i8; 64] {
        let mut board = [0; 64];
        let dictionary: HashMap<char, i8> = [
            ('k', Piece::King as i8),
            ('p', Piece::Pawn as i8),
            ('n', Piece::Knight as i8),
            ('b', Piece::Bishop as i8),
            ('r', Piece::Rook as i8),
            ('q', Piece::Queen as i8),
        ]
        .iter()
        .cloned()
        .collect();

        let board_string = fen_string.split_whitespace().nth(0).unwrap();
        let mut file: i8 = 0;
        let mut rank: i8 = 7;

        for symbol in board_string.to_string().chars() {
            if symbol == '/' {
                file = 0;
                rank -= 1;
            } else {
                if symbol.is_digit(10) {
                    file += symbol.to_digit(10).unwrap() as i8;
                } else {
                    let color = if symbol.is_uppercase() {
                        Piece::white()
                    } else {
                        Piece::black()
                    };
                    let piece = *dictionary.get(&symbol.to_ascii_lowercase()).unwrap();
                    board[(rank * 8 + file) as usize] = piece | color;
                    file += 1;
                }
            }
        }
        board
    }

    pub fn encode(board: [i8; 64]) -> String {
        unimplemented!()
    }
}
