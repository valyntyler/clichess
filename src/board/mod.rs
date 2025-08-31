use std::fmt::Display;

use colored::{Color, ColoredString, Colorize};

use crate::piece::{
    Piece,
    char::{NerdFontNotation, PieceChar},
    color::PieceColor,
    shape::PieceShape,
};

pub struct Board([[Option<Piece>; 8]; 8]);

impl Default for Board {
    fn default() -> Self {
        Self([
            [
                Some(Piece::new(PieceColor::Black, PieceShape::Rook)),
                Some(Piece::new(PieceColor::Black, PieceShape::Knight)),
                Some(Piece::new(PieceColor::Black, PieceShape::Bishop)),
                Some(Piece::new(PieceColor::Black, PieceShape::Queen)),
                Some(Piece::new(PieceColor::Black, PieceShape::King)),
                Some(Piece::new(PieceColor::Black, PieceShape::Bishop)),
                Some(Piece::new(PieceColor::Black, PieceShape::Knight)),
                Some(Piece::new(PieceColor::Black, PieceShape::Rook)),
            ],
            [Some(Piece::new(PieceColor::Black, PieceShape::Pawn)); 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            [Some(Piece::new(PieceColor::White, PieceShape::Pawn)); 8],
            [
                Some(Piece::new(PieceColor::White, PieceShape::Rook)),
                Some(Piece::new(PieceColor::White, PieceShape::Knight)),
                Some(Piece::new(PieceColor::White, PieceShape::Bishop)),
                Some(Piece::new(PieceColor::White, PieceShape::Queen)),
                Some(Piece::new(PieceColor::White, PieceShape::King)),
                Some(Piece::new(PieceColor::White, PieceShape::Bishop)),
                Some(Piece::new(PieceColor::White, PieceShape::Knight)),
                Some(Piece::new(PieceColor::White, PieceShape::Rook)),
            ],
        ])
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rank, row) in self.0.iter().enumerate() {
            for (file, cell) in row.iter().enumerate() {
                let bg = if (rank + file) % 2 == 0 {
                    Color::BrightBlack
                } else {
                    Color::White
                };

                let char = if let Some(piece) = cell {
                    NerdFontNotation(piece.shape)
                        .char()
                        .to_string()
                        .color(match piece.color {
                            PieceColor::White => Color::BrightWhite,
                            PieceColor::Black => Color::Black,
                        })
                } else {
                    ColoredString::from(" ")
                };

                let square = format!(" {} ", char).on_color(bg);
                write!(f, "{}", square)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
