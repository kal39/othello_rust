use crate::game::bot::Bot;
use crate::game::human::Human;
use crate::game::main_loop;

mod game;

fn main() {
	let depth = 3;
	// main_loop::start((Bot::black(depth), Bot::white(depth)));
	main_loop::start((Bot::new(depth), Bot::new(depth)));
}
