use crate::ga::population::Population;
use crate::game::bot::Bot;
use crate::game::human::Human;
use crate::game::main_loop;

mod ga;
mod game;

fn main() {
	// train();
	play_bot_vs_bot_game();
}

fn play_bot_vs_bot_game() {
	let depth = 2;
	let weights_b = [100, -20, 10, 5, -50, -2, -2, -1, -1, -1].to_vec();
	let weights_w = [75, -2, 36, -31, 106, 19, -81, -32, -42, 124].to_vec();
	main_loop::start((
		Bot::new(depth, weights_b.clone()),
		Bot::new(depth, weights_w.clone()),
	));
}

fn play_human_vs_bot_game() {
	let depth = 3;
	let weights = [100, -20, 10, 5, -50, -2, -2, -1, -1, -1].to_vec();
	main_loop::start((Human::new(), Bot::new(depth, weights.clone())));
}

fn train() {
	let mut population = Population::new(20);
	population.evolve(100, 2, 2, 0.01);
}
