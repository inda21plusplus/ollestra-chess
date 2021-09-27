#[derive(Debug)]
pub(crate) struct MoveOffset;

impl MoveOffset {
    // https://www.chessprogramming.org/Rays
    pub const STRAIGHT: [i8; 4] = [8, -8, -1, 1];
    pub const ASKEW: [i8; 4] = [7, -7, 9, -9];
    pub const KNIGHT: [i8; 8] = [15, 17, 6, 10, -10, -6, -15, -17];
    pub const PAWN_WHITE: [i8; 3] = [8, 7, 9];
    pub const PAWN_BLACK: [i8; 3] = [-8, -7, -9];
}
