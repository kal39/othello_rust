use super::cell::Cell;
use super::game_state::GameState;
use super::player::Player;

pub struct Bot {
	weights: [[f64; 8]; 8],
	max_depth: i32,
	player: Cell,
}

impl Player for Bot {
	fn make_move(&mut self, g: &GameState) -> (i32, i32) {
		self.player = g.active_player();

		let mut max = f64::MIN;
		let mut max_move = (-1, -1);

		for i in 0..8 {
			for j in 0..8 {
				if g.placable((i, j)) {
					let mut g2 = g.clone();
					g2.place((i, j));
					let score = self.minimax(&g2, 1, f64::MIN, f64::MAX, false);
					if score > max {
						max = score;
						max_move = (i, j);
					}
				}
			}
		}

		println!(
			"x (col): {}, y (row): {}, score: {}",
			max_move.0, max_move.1, max
		);
		max_move
	}
}

impl Bot {
	pub fn new(max_depth: i32) -> Bot {
		Bot {
			weights: [
				[100.0, -20.0, 10.0, 5.0, 5.0, 10.0, -20.0, 100.0],
				[-20.0, -50.0, -2.0, -2.0, -2.0, -2.0, -50.0, -20.0],
				[10.0, -2.0, -1.0, -1.0, -1.0, -1.0, -2.0, 10.0],
				[5.0, -2.0, -1.0, -1.0, -1.0, -1.0, -2.0, 5.0],
				[5.0, -2.0, -1.0, -1.0, -1.0, -1.0, -2.0, 5.0],
				[10.0, -2.0, -1.0, -1.0, -1.0, -1.0, -2.0, 10.0],
				[-20.0, -50.0, -2.0, -2.0, -2.0, -2.0, -50.0, -20.0],
				[100.0, -20.0, 10.0, 5.0, 5.0, 10.0, -20.0, 100.0],
			],
			max_depth: max_depth,
			player: Cell::Empty,
		}
	}

	fn minimax(&self, g: &GameState, k: i32, mut alpha: f64, mut beta: f64, maxing: bool) -> f64 {
		if k > self.max_depth || g.over() {
			return g.evaluate(self.player, self.weights);
		}

		match maxing {
			true => {
				let mut max = f64::MIN;
				for i in 0..8 {
					for j in 0..8 {
						if g.placable((i, j)) {
							let mut g2 = g.clone();
							g2.place((i, j));
							let score = self.minimax(&g2, k + 1, alpha, beta, false);
							if score > max {
								max = score;
							}
							if max > alpha {
								alpha = max;
							}
							if alpha >= beta {
								return max;
							}
						}
					}
				}
				max
			}
			false => {
				let mut min = f64::MAX;
				for i in 0..8 {
					for j in 0..8 {
						if g.placable((i, j)) {
							let mut g2 = g.clone();
							g2.place((i, j));
							let score = self.minimax(&g2, k + 1, alpha, beta, true);
							if score < min {
								min = score;
							}
							if min < beta {
								beta = min;
							}
							if alpha >= beta {
								return min;
							}
						}
					}
				}
				min
			}
		}
	}
}
