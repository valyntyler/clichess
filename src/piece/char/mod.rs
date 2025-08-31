use crate::piece::shape::PieceShape;

pub trait PieceChar {
    fn char(&self) -> char;
}

pub struct AsciiNotation(pub PieceShape);
pub struct UnicodeNotation(pub PieceShape);
pub struct NerdFontNotation(pub PieceShape);

impl PieceChar for AsciiNotation {
    fn char(&self) -> char {
        match self.0 {
            PieceShape::Pawn => 'p',
            PieceShape::Knight => 'n',
            PieceShape::Bishop => 'b',
            PieceShape::Rook => 'r',
            PieceShape::Queen => 'q',
            PieceShape::King => 'k',
        }
    }
}

impl PieceChar for UnicodeNotation {
    fn char(&self) -> char {
        match self.0 {
            PieceShape::Pawn => '♙',
            PieceShape::Knight => '♘',
            PieceShape::Bishop => '♗',
            PieceShape::Rook => '♖',
            PieceShape::Queen => '♕',
            PieceShape::King => '♔',
        }
    }
}

impl PieceChar for NerdFontNotation {
    fn char(&self) -> char {
        match self.0 {
            PieceShape::Pawn => '',
            PieceShape::Knight => '',
            PieceShape::Bishop => '',
            PieceShape::Rook => '',
            PieceShape::Queen => '',
            PieceShape::King => '',
        }
    }
}
