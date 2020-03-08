use crate::square::Rank;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
  White,
  Black,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
  WhitePawn,
  BlackPawn,
  Knight,
  Bishop,
  Rook,
  Queen,
  King,
  Nil,
}

pub const NUM_PIECES: usize = 7;

impl Color {
  pub fn other(self) -> Self {
    match self {
      Color::White => Color::Black,
      Color::Black => Color::White,
    }
  }

  pub fn home_rank(self) -> Rank {
    match self {
      Color::White => Rank::R1,
      Color::Black => Rank::R8,
    }
  }
}
