use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum ChessError {
    #[snafu(display("This move i considered illegal"))]
    IllegalMove,
    #[snafu(display("This move is outside of the board"))]
    MoveOutsideBoard,
    #[snafu(display("You can not capture your pieces"))]
    CaptureTeamPiece,
    #[snafu(display("There is no piece at ({},{})", x, y))]
    NullPosition { x: u8, y: u8 },
}