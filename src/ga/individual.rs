extern crate rand;

#[derive(Clone)]
pub struct Individual {
	chromosome: Vec<i8>,
	fitness: i32,
}

impl Individual {
	pub fn new(chromosome_count: i32) -> Self {
		let mut individual = Individual {
			chromosome: Vec::new(),
			fitness: 0,
		};
		for _ in 0..chromosome_count {
			individual.chromosome.push(rand::random());
		}
		individual
	}

	pub fn get_phenotype(&self) -> Vec<i8> {
		self.chromosome.clone()
	}

	pub fn add_fitness(&mut self, value: i32) {
		self.fitness += value;
	}

	pub fn get_fitness(&self) -> i32 {
		self.fitness
	}

	pub fn crossover(&self, other: Individual) -> Individual {
		let len = self.chromosome.len();
		let mut chromosome = self.chromosome[0..len / 2].to_vec();
		chromosome.append(&mut other.chromosome[len / 2..len].to_vec());
		Individual {
			chromosome: chromosome,
			fitness: 0,
		}
	}

	pub fn mutate(&mut self, chance: f64) {
		for i in 0..self.chromosome.len() {
			if rand::random::<f64>() < chance {
				self.chromosome[i] = rand::random();
			}
		}
	}
}
