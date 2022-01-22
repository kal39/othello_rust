use std::fmt;
use std::ops::Neg;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
	Empty,
	Black,
	White,
}

impl Neg for Cell {
	type Output = Cell;

	fn neg(self) -> Cell {
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
			Cell::Empty => write!(f, "\x1B[90m·\x1B[39m"),
			Cell::Black => write!(f, "\x1B[30m●\x1B[39m"),
			Cell::White => write!(f, "\x1B[97m●\x1B[39m"),
		}
	}
}
