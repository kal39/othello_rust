use super::game_state::GameState;
use std::io::{self, Write};

fn read_line() -> String {
	let mut buffer = String::new();
	io::stdin().read_line(&mut buffer).unwrap();
	buffer
}

fn get_input(g: &GameState) -> (i32, i32) {
	print!("{:?}\n  x (col): ", g.current_player());
	io::stdout().flush().unwrap();
	let col = read_line().trim().parse::<i32>().unwrap();
	print!("  y (row): ");
	io::stdout().flush().unwrap();
	let row = read_line().trim().parse::<i32>().unwrap();
	(row - 1, col - 1)
}

pub fn start() {
	let mut g = GameState::new();
	loop {
		println!("{}", g);
		while !g.place(get_input(&g)) {}
	}
}
