#[derive(Debug)]
pub(crate) struct MoveOffset;

impl MoveOffset {
    // https://www.chessprogramming.org/Rays
    pub const STRAIGHT: [i8; 4] = [8, -8, 1, -1];
    pub const ASKEW: [i8; 4] = [7, -7, 9, -9];
}
