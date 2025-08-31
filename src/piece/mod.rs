pub mod char;
pub mod color;
pub mod shape;

use crate::piece::color::PieceColor;
use crate::piece::shape::PieceShape;

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
