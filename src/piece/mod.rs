#[derive(Clone, Copy)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Clone, Copy)]
pub enum PieceShape {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Clone, Copy)]
pub struct Piece {
    color: PieceColor,
    shape: PieceShape,
}
