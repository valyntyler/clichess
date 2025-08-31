use std::fmt::Display;

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
