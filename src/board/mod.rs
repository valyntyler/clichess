use std::fmt::Display;

use colored::{Color, Colorize};

use crate::piece::{Piece, PieceColor, PieceShape};

pub struct Board([[Option<Piece>; 8]; 8]);

impl Default for Board {
    fn default() -> Self {
        Self([[Some(Piece::new(PieceColor::Black, PieceShape::King)); 8]; 8])
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (rank, row) in self.0.iter().enumerate() {
            for (file, piece) in row.iter().enumerate() {
                let fg = if rank < 4 {
                    Color::Black
                } else {
                    Color::BrightWhite
                };

                let bg = if (rank + file) % 2 == 0 {
                    Color::BrightBlack
                } else {
                    Color::White
                };

                let square = format!(
                    " {} ",
                    if let Some(p) = piece {
                        p.to_string()
                    } else {
                        " ".to_string()
                    }
                )
                .color(fg)
                .on_color(bg);

                write!(f, "{}", square)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
