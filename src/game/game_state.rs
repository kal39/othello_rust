use super::cell::Cell;
use std::fmt;

pub struct GameState {
	turn: i32,
	board: [[Cell; 8]; 8],
}

impl GameState {
	pub fn new() -> Self {
		let mut g = GameState {
			turn: 1,
			board: [[Cell::Empty; 8]; 8],
		};
		g.board[3][3] = Cell::Black;
		g.board[4][4] = Cell::Black;
		g.board[3][4] = Cell::White;
		g.board[4][3] = Cell::White;
		g
	}

	fn get(&self, pos: (i32, i32)) -> Cell {
		self.board[pos.0 as usize][pos.1 as usize]
	}

	fn set(&mut self, pos: (i32, i32), piece: Cell) {
		self.board[pos.0 as usize][pos.1 as usize] = piece;
	}

	pub fn current_player(&self) -> Cell {
		if self.turn % 2 == 1 {
			Cell::Black
		} else {
			Cell::White
		}
	}

	pub fn place(&mut self, pos: (i32, i32)) -> bool {
		let mut placed = false;
		let mut locations: Vec<(i32, i32)> = Vec::new();

		if pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 && self.get(pos) == Cell::Empty {
			for i in -1..2 {
				for j in -1..2 {
					let (check, new_locs) = self.check_dir(pos, (i, j));
					if check {
						placed = true;
						locations.extend(new_locs);
					}
				}
			}
		}
		if placed {
			self.set((pos.0, pos.1), self.current_player());
			for piece in locations {
				self.set(piece, self.current_player());
			}

			self.turn += 1;
			true
		} else {
			false
		}
	}

	fn placable(&self, pos: (i32, i32)) -> bool {
		if pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8 && self.get(pos) == Cell::Empty {
			for i in -1..2 {
				for j in -1..2 {
					let (check, _pieces) = self.check_dir(pos, (i, j));
					if check {
						return true;
					}
				}
			}
		}
		false
	}

	fn check_dir(&self, pos: (i32, i32), dir: (i32, i32)) -> (bool, Vec<(i32, i32)>) {
		let mut next = (pos.0 + dir.0, pos.1 + dir.1);
		let mut cells = vec![next];

		if next.0 >= 0
			&& next.0 < 8
			&& next.1 >= 0
			&& next.1 < 8
			&& self.get(next) == -self.current_player()
		{
			while next.0 >= 1 && next.0 < 7 && next.1 >= 1 && next.1 < 7 {
				next = (next.0 + dir.0, next.1 + dir.1);
				if self.get(next) == self.current_player() {
					return (true, cells);
				} else {
					cells.push(next);
				}
			}
		}
		(false, Vec::new())
	}
}

impl fmt::Display for GameState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "turn {}, {:?}:\n\n  ", self.turn, self.current_player())?;
		for i in 1..8 + 1 {
			write!(f, "{} ", i)?;
		}
		write!(f, "\n")?;
		for i in 0..8 {
			for j in 0..8 {
				if j == 0 {
					write!(f, "{} ", i + 1)?;
				}
				if self.placable((i, j)) {
					write!(f, "▪ ")?;
				} else {
					write!(f, "{} ", self.get((i, j)))?;
				}
			}
			write!(f, "\n")?;
		}
		write!(f, "")
	}
}
