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
        let pawns = [PieceShape::Pawn; 8];
        let pieces = [
            PieceShape::Rook,
            PieceShape::Knight,
            PieceShape::Bishop,
            PieceShape::Queen,
            PieceShape::King,
            PieceShape::Bishop,
            PieceShape::Knight,
            PieceShape::Rook,
        ];

        Self([
            pieces.map(|shape| Some(Piece::new(PieceColor::Black, shape))),
            pawns.map(|shape| Some(Piece::new(PieceColor::Black, shape))),
            [None; 8],
            [None; 8],
            [None; 8],
            [None; 8],
            pawns.map(|shape| Some(Piece::new(PieceColor::White, shape))),
            pieces.map(|shape| Some(Piece::new(PieceColor::White, shape))),
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
