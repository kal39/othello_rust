use super::player::Player;
use crate::game::game_state::GameState;
use std::io::{self, Write};

pub struct Human {}

impl Player for Human {
	fn make_move(&mut self, g: &GameState) -> (i32, i32) {
		Human::get_input(g)
	}
}

impl Human {
	pub fn new() -> Self {
		Human {}
	}

	fn read_line() -> String {
		let mut buffer = String::new();
		io::stdin().read_line(&mut buffer).unwrap();
		buffer
	}

	fn get_input(g: &GameState) -> (i32, i32) {
		print!("{:?}\n  x (col): ", g.active_player());
		io::stdout().flush().unwrap();
		let col = Human::read_line().trim().parse::<i32>().unwrap();
		print!("  y (row): ");
		io::stdout().flush().unwrap();
		let row = Human::read_line().trim().parse::<i32>().unwrap();
		(row - 1, col - 1)
	}
}
