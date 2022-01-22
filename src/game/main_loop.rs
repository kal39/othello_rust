use super::cell::Cell;
use super::game_state::GameState;
use super::player::Player;

pub fn start(mut players: (impl Player, impl Player)) {
	let mut g = GameState::new();
	while !g.over() {
		println!("\n{}", g);
		match g.active_player() {
			Cell::Black => g.place(players.0.make_move(&g)),
			Cell::White => g.place(players.1.make_move(&g)),
			_ => false,
		};
	}

	println!("\n==== RESULT ====\n");
	println!("{}", g);
}
