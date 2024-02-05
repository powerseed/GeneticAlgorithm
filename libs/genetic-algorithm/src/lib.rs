use rand::prelude::*;
pub fn evolve(population: &[Individual]) -> Vec<Individual> {
    (0..population.len())
        .map(|_| {
            let mut rng = thread_rng();
            let parent_one = select(&rng, population);
            let parent_two = select(&rng, population);

            let mut child = crossover(&mut rng, parent_one, parent_two);

            mutate(&rng, &mut child);

            child
        })
        .collect()
}
pub fn select<'a>(rng: &mut dyn RngCore, population: &'a [Individual]) -> &'a Individual {
    population.choose_weighted(rng, |individual| individual.fitness).unwrap()
}
pub fn crossover(rng: &mut dyn RngCore, parent_one: &Individual, parent_two: &Individual) -> Individual {
    assert_eq!(parent_one.chromosomes.len(), parent_two.chromosomes.len());

    let mut child = Individual {
        fitness: 0.0,
        chromosomes: vec![]
    };

    parent_one.chromosomes
        .iter()
        .zip(parent_two.chromosomes.iter())
        .for_each(|(&first_gene, &second_gene)| {
            if rng.gen_bool(0.5) {
                child.chromosomes.push(first_gene);
            }
            else {
                child.chromosomes.push(second_gene);
            }
        });

    child
}
pub fn mutate(rng: &mut dyn RngCore, individual: &mut Individual) {
    let chance = 0.5;
    let coeff = 2.0;

    individual.chromosomes.iter().for_each(|&chromosome| {
        let sign = if rng.gen_bool(0.5) { 1 } else { -1 };

        if rng.gen_bool(chance) {
           *chromosome += chromosome * sign * coeff * rng.gen::<f32>();
        }
    })
}
pub struct Individual {
    fitness: f32,
    chromosomes: Vec<f32>
}