pub mod color;
pub mod shape;

use std::fmt::Display;

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

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.shape {
                PieceShape::Pawn => '',
                PieceShape::Knight => '',
                PieceShape::Bishop => '',
                PieceShape::Rook => '',
                PieceShape::Queen => '',
                PieceShape::King => '',
            }
        )
    }
}
