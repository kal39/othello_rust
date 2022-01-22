use super::game_state::GameState;

pub trait Player {
	fn make_move(&mut self, g: &GameState) -> (i32, i32);
}
