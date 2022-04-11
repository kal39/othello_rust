use super::individual::Individual;
use crate::game::bot::Bot;
use crate::game::main_loop;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::fmt;

pub struct Population {
	individuals: Vec<Individual>,
}

impl Population {
	pub fn new(population_size: i32) -> Self {
		let mut population = Population {
			individuals: Vec::new(),
		};
		for _ in 0..population_size {
			population.individuals.push(Individual::new(10));
		}
		population
	}

	pub fn evolve(
		&mut self,
		generations: i32,
		iterations: i32,
		max_depth: i32,
		mutation_rate: f64,
	) {
		let population_size = self.individuals.len() as i32;
		for i in 0..generations {
			self.calculate_fitness(iterations, max_depth);
			println!("generation {}\n{}\n", i, self);
			self.select(max_depth);
			self.crossover(population_size);
			self.mutate(mutation_rate);
		}
	}

	fn calculate_fitness(&mut self, iterations: i32, max_depth: i32) {
		self.individuals.shuffle(&mut thread_rng());

		for i in 1..iterations as usize {
			for j in 0..self.individuals.len() {
				let opponent_idx = if j + i < self.individuals.len() {
					j + i
				} else {
					j + i - self.individuals.len()
				};

				let (a1, b1) = main_loop::duel((
					Bot::new(max_depth, self.individuals[j].get_phenotype()),
					Bot::new(max_depth, self.individuals[opponent_idx].get_phenotype()),
				));
				let (b2, a2) = main_loop::duel((
					Bot::new(max_depth, self.individuals[opponent_idx].get_phenotype()),
					Bot::new(max_depth, self.individuals[j].get_phenotype()),
				));

				self.individuals[j].add_fitness(a1 + a2);
				self.individuals[opponent_idx].add_fitness(b1 + b2);
			}
		}

		self.individuals
			.sort_by(|a, b| b.get_fitness().cmp(&a.get_fitness()));
	}

	fn select(&mut self, size: i32) {
		self.individuals
			.drain(size as usize..self.individuals.len());
	}

	// pub fn select(&mut self, max_depth: i32) {
	// 	self.individuals.shuffle(&mut thread_rng());
	// 	let mut survivors = Vec::new();

	// 	for i in 0..self.individuals.len() / 2 {
	// 		let result = main_loop::duel((
	// 			Bot::new(max_depth, self.individuals[i * 2].get_phenotype()),
	// 			Bot::new(max_depth, self.individuals[i * 2 + 1].get_phenotype()),
	// 		)) - main_loop::duel((
	// 			Bot::new(max_depth, self.individuals[i * 2 + 1].get_phenotype()),
	// 			Bot::new(max_depth, self.individuals[i * 2].get_phenotype()),
	// 		));
	// 		if result >= 0 {
	// 			survivors.push(self.individuals[i * 2].clone());
	// 		} else {
	// 			survivors.push(self.individuals[i * 2 + 1].clone());
	// 		}
	// 	}

	// 	// self.individuals
	// 	// 	.sort_by(|a, b| a.get_fitness().cmp(&b.get_fitness()));
	// }

	pub fn crossover(&mut self, size: i32) {
		let mut children = Vec::new();
		for _ in 0..size {
			children.push(self.rand_individual().crossover(self.rand_individual()));
		}
		self.individuals = children;
	}

	pub fn mutate(&mut self, chance: f64) {
		for i in &mut self.individuals {
			i.mutate(chance);
		}
	}

	fn rand_individual(&mut self) -> Individual {
		self.individuals[rand::thread_rng().gen_range(0, self.individuals.len())].clone()
	}
}

impl fmt::Display for Population {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "individuals:\n")?;
		for i in 0..self.individuals.len() {
			write!(
				f,
				"[{}] ({}): {:?}\n",
				i,
				self.individuals[i].get_fitness(),
				self.individuals[i].get_phenotype()
			)?;
		}
		write!(f, "\n")
	}
}
