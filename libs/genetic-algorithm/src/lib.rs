use rand::prelude::*;
pub struct GeneticAlgorithm;

impl GeneticAlgorithm {
    pub fn new() -> Self {
        Self{}
    }

    pub fn evolve<I>(&self, rng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual
    {
        (0..population.len())
            .map(|_| {
                let parent_one = GeneticAlgorithm::select_one_individual_based_on_fitness(rng, population);
                let parent_one_chromosomes = parent_one.chromosomes();
                let parent_two = GeneticAlgorithm::select_one_individual_based_on_fitness(rng, population);
                let parent_two_chromosomes = parent_two.chromosomes();

                let mut child_chromosomes = GeneticAlgorithm::crossover_breed(rng, parent_one_chromosomes, parent_two_chromosomes);

                GeneticAlgorithm::mutate(rng, &mut child_chromosomes);
                I::new(child_chromosomes)
            })
            .collect()
    }

    fn select_one_individual_based_on_fitness<'a, I>(rng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where I: Individual
    {
        population
            .choose_weighted(rng, |individual| individual.fitness())
            .expect("Empty population. ")
    }

    fn crossover_breed (rng: &mut dyn RngCore, parent_one_chromosomes: &[f32], parent_two_chromosomes: &[f32]) -> Vec<f32> {
        assert_eq!(parent_one_chromosomes.len(), parent_two_chromosomes.len());

        let mut child_chromosomes = Vec::new();

        parent_one_chromosomes
            .iter()
            .zip(parent_two_chromosomes.iter())
            .for_each(|(first_chromosome, second_chromosome)| {
                if rng.gen_bool(0.5) {
                    child_chromosomes.push(*first_chromosome);
                }
                else {
                    child_chromosomes.push(*second_chromosome);
                }
            });

        child_chromosomes
    }

    fn mutate(rng: &mut dyn RngCore, chromosomes: &mut [f32]) {
        let chance = 0.01;
        let coeff = 0.3;

        for chromosome in chromosomes.iter_mut() {
            let sign = if rng.gen_bool(0.5) { 1.0 } else { -1.0 };

            if rng.gen_bool(chance as _) {
                *chromosome += sign * coeff * rng.gen::<f32>();
            }
        };
    }
}
pub trait Individual {
    fn new(chromosomes: Vec<f32>) -> Self;
    fn fitness(&self) -> f32;
    fn chromosomes(&self) -> &Vec<f32>;
}

