#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub rank: i8,
    pub file: i8,
}

impl Square {
    pub const MAX_SIZE: i8 = 63;

    pub fn new(rank: i8, file: i8) -> Self {
        Self { rank, file }
    }

    pub fn to_i8(&self) -> i8 {
        self.rank * 8 + self.file
    }

    pub fn from_i8(square: i8) -> Self {
        Self {
            rank: square / 8,
            file: square % 8,
        }
    }

    pub fn is_outside(&self) -> bool {
        self.rank > 7i8 || self.file > 7i8
    }
}
