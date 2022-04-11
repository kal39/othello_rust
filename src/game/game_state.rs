use super::cell::Cell;
use std::fmt;

#[derive(Clone)]
pub struct GameState {
	turn: i32,
	game_over: bool,
	board: [[Cell; 8]; 8],
}

impl GameState {
	pub fn new() -> Self {
		let mut g = GameState {
			turn: 1,
			game_over: false,
			board: [[Cell::Empty; 8]; 8],
		};
		g.board[3][3] = Cell::Black;
		g.board[4][4] = Cell::Black;
		g.board[3][4] = Cell::White;
		g.board[4][3] = Cell::White;
		g
	}

	pub fn over(&self) -> bool {
		self.game_over
	}

	pub fn active_player(&self) -> Cell {
		if self.turn % 2 == 1 {
			Cell::Black
		} else {
			Cell::White
		}
	}

	pub fn inactive_player(&self) -> Cell {
		-self.active_player()
	}

	pub fn placable(&self, pos: (i32, i32)) -> bool {
		if GameState::in_board(pos) && self.get(pos) == Cell::Empty {
			for i in [-1, 0, 1] {
				for j in [-1, 0, 1] {
					if self.check_dir(pos, (i, j)).0 {
						return true;
					}
				}
			}
		}
		false
	}

	pub fn place(&mut self, pos: (i32, i32)) -> bool {
		let mut placable = false;
		let mut locations = Vec::new();

		if GameState::in_board(pos) && self.get(pos) == Cell::Empty {
			for i in [-1, 0, 1] {
				for j in [-1, 0, 1] {
					let (check, new_locs) = self.check_dir(pos, (i, j));
					if check {
						placable = true;
						locations.extend(new_locs);
					}
				}
			}
		}
		if placable {
			self.set((pos.0, pos.1), self.active_player());
			for piece in locations {
				self.set(piece, self.active_player());
			}
			self.turn += 1;

			if self.skip_necessary() {
				self.turn += 1;
				if self.skip_necessary() {
					self.game_over = true;
				}
			}

			true
		} else {
			false
		}
	}

	pub fn evaluate(&self, player: Cell, weights: [[i8; 8]; 8]) -> f64 {
		let mut tot = 0.0;
		for i in 0..8 {
			for j in 0..8 {
				tot += if self.board[i][j] == player {
					weights[i][j] as f64
				} else if self.board[i][j] == -player {
					-(weights[i][j] as f64)
				} else {
					0.0
				}
			}
		}
		tot
	}

	pub fn score(&self, player: Cell) -> i32 {
		let mut tot = 0;
		for i in 0..8 {
			for j in 0..8 {
				if self.board[i][j] == player {
					tot += 1;
				}
			}
		}
		tot
	}

	fn skip_necessary(&self) -> bool {
		for i in 0..8 {
			for j in 0..8 {
				if self.placable((i, j)) {
					return false;
				}
			}
		}
		true
	}

	fn get(&self, pos: (i32, i32)) -> Cell {
		self.board[pos.0 as usize][pos.1 as usize]
	}

	fn set(&mut self, pos: (i32, i32), piece: Cell) {
		self.board[pos.0 as usize][pos.1 as usize] = piece;
	}

	fn in_board(pos: (i32, i32)) -> bool {
		pos.0 >= 0 && pos.0 < 8 && pos.1 >= 0 && pos.1 < 8
	}

	fn check_dir(&self, pos: (i32, i32), dir: (i32, i32)) -> (bool, Vec<(i32, i32)>) {
		let mut next = (pos.0 + dir.0, pos.1 + dir.1);
		let mut cells = vec![next];

		if !GameState::in_board(next) || self.get(next) != self.inactive_player() {
			return (false, Vec::new());
		}

		next = (next.0 + dir.0, next.1 + dir.1);

		while next.0 >= 0 && next.0 < 8 && next.1 >= 0 && next.1 < 8 {
			if self.get(next) == self.active_player() {
				return (true, cells);
			} else {
				cells.push(next);
			}
			next = (next.0 + dir.0, next.1 + dir.1);
		}

		(false, Vec::new())
	}
}

impl fmt::Display for GameState {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "turn {}, {:?}:\n", self.turn, self.active_player())?;
		write!(
			f,
			"score: (black: {}, white: {})\n\n  ",
			self.score(Cell::Black),
			self.score(Cell::White)
		)?;
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
					write!(f, "\x1B[42m\x1B[31m▪ \x1B[49m\x1B[39m")?;
				} else {
					write!(f, "\x1B[42m{} \x1B[49m", self.get((i, j)))?;
				}
			}
			write!(f, "\n")?;
		}
		write!(f, "")
	}
}
