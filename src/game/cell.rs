use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
	Empty,
	Black,
	White,
}

impl ops::Neg for Cell {
	type Output = Self;
	fn neg(self) -> Self::Output {
		match self {
			Cell::Empty => Cell::Empty,
			Cell::Black => Cell::White,
			Cell::White => Cell::Black,
		}
	}
}

impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Cell::Empty => write!(f, "·"),
			Cell::Black => write!(f, "○"),
			Cell::White => write!(f, "●"),
		}
	}
}
