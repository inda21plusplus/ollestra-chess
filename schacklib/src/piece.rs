use bitflags::bitflags;

// Bit implementation: https://www.chessprogramming.org/General_Setwise_Operations

bitflags! {}

pub(crate) enum Piece {
    None = 0,   // 00000
    Pawn = 1,   // 00001  --> MOVES
    Rook = 2,   // 00010  --> MOVES
    Knight = 3, // 00011  --> KLAR
    Bishop = 4, // 00100  --> KLAR
    Queen = 5,  // 00101  --> KLAR
    King = 6,   // 00110  --> MOVES
    White = 8,  // 01000
    Black = 16, // 10000
}

/*
    Examples:
        Pawn | White = 00001 | 01000 = 01001 => "White Pawn"
        King | Black = 00110 | 10000 = 10110 => "Black King"
*/

impl Default for Piece {
    fn default() -> Self {
        Self::None
    }
}

impl Piece {
    const WHITE_MASK: i8 = 8;
    const BLACK_MASK: i8 = 16;
    const COLOR_MASK: i8 = 8 | 16;

    pub fn none() -> i8 {
        Piece::None as i8
    }

    pub fn pawn(is_white: bool) -> i8 {
        if is_white {
            Piece::Pawn as i8 | Piece::White as i8
        } else {
            Piece::Pawn as i8 | Piece::Black as i8
        }
    }

    pub fn rook(is_white: bool) -> i8 {
        if is_white {
            Piece::Rook as i8 | Piece::White as i8
        } else {
            Piece::Rook as i8 | Piece::Black as i8
        }
    }

    pub fn knight(is_white: bool) -> i8 {
        if is_white {
            Piece::Knight as i8 | Piece::White as i8
        } else {
            Piece::Knight as i8 | Piece::Black as i8
        }
    }

    pub fn bishop(is_white: bool) -> i8 {
        if is_white {
            Piece::Bishop as i8 | Piece::White as i8
        } else {
            Piece::Bishop as i8 | Piece::Black as i8
        }
    }

    pub fn queen(is_white: bool) -> i8 {
        if is_white {
            Piece::Queen as i8 | Piece::White as i8
        } else {
            Piece::Queen as i8 | Piece::Black as i8
        }
    }

    pub fn king(is_white: bool) -> i8 {
        if is_white {
            Piece::King as i8 | Piece::White as i8
        } else {
            Piece::King as i8 | Piece::Black as i8
        }
    }

    pub fn white() -> i8 {
        Piece::White as i8
    }

    pub fn black() -> i8 {
        Piece::Black as i8
    }

    pub fn get_color(piece: i8) -> i8 {
        piece & Piece::COLOR_MASK
    }

    pub fn get_opponent_color(piece: i8) -> i8 {
        Self::bit_swap(piece & Piece::COLOR_MASK, 3, 4)
    }

    pub fn cmp_color(piece1: i8, color: i8) -> bool {
        piece1 & Piece::COLOR_MASK == color
    }

    // https://www.geeksforgeeks.org/how-to-swap-two-bits-in-a-given-integer/
    fn bit_swap(n: i8, p1: i8, p2: i8) -> i8 {
        let bit1: i8 = (&n >> p1) & 1;
        let bit2: i8 = (&n >> p2) & 1;
        let xor: i8 = bit1 ^ bit2;

        n ^ ((xor << p1) | (xor << p2))
    }
}
