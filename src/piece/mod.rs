pub enum PieceColor {
    White,
    Black,
}

pub enum PieceShape {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    color: PieceColor,
    shape: PieceShape,
}
