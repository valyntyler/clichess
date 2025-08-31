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
    pub color: PieceColor,
    pub shape: PieceShape,
}

impl Piece {
    pub fn new(color: PieceColor, shape: PieceShape) -> Self {
        Self { color, shape }
    }
}
