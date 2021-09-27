use bitflags::bitflags;

// Bit implementation: https://www.chessprogramming.org/General_Setwise_Operations

bitflags! {}

pub(crate) enum Piece {
    None = 0,    // 000000
    Pawn = 1,    // 000001  --> MOVES
    Rook = 2,    // 000010  --> KLAR
    Knight = 3,  // 000011  --> KLAR
    Bishop = 4,  // 000100  --> KLAR
    Queen = 5,   // 000101  --> KLAR
    King = 6,    // 000110  --> MOVES
    White = 8,   // 001000
    Black = 16,  // 010000
    Pinned = 32, // 100000
}

/*
    Examples:
        Pawn | White = 000001 | 001000 = 001001 => "White Pawn"
        King | Black = 000110 | 010000 = 010110 => "Black King"
*/

impl Default for Piece {
    fn default() -> Self {
        Self::None
    }
}

impl Piece {
    pub const PIECE_MASK: i8 = 7;
    const WHITE_MASK: i8 = 8;
    const BLACK_MASK: i8 = 16;
    const COLOR_MASK: i8 = 8 | 16;
    const PINNNED_MASK: i8 = 32;

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

    pub fn pinned() -> i8 {
        Piece::Pinned as i8
    }

    pub fn get_color(piece: i8) -> i8 {
        piece & Piece::COLOR_MASK
    }

    pub fn get_opponent_color(piece: i8) -> i8 {
        Self::bit_swap(piece & Piece::COLOR_MASK, 3, 4)
    }

    pub fn cmp_color(piece: i8, color: i8) -> bool {
        piece & Piece::COLOR_MASK == color
    }

    pub fn is_pinned(piece: i8) -> bool {
        piece & (1 << 6) != 0
    }

    // https://www.geeksforgeeks.org/how-to-swap-two-bits-in-a-given-integer/
    fn bit_swap(n: i8, p1: i8, p2: i8) -> i8 {
        let bit1: i8 = (&n >> p1) & 1;
        let bit2: i8 = (&n >> p2) & 1;
        let xor: i8 = bit1 ^ bit2;

        n ^ ((xor << p1) | (xor << p2))
    }
}
