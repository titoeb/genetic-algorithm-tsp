use genetic_algo::distance_mat::DistanceMat;
use genetic_algo::population::Population;
use std::fs;

#[test]
fn run_evolution() {
    let n_generations = 10;
    let size_generation = 20;
    // End-to-end test: does the error of the solution get down?
    let distances = DistanceMat::new(
        fs::read_to_string("tests/test-data/distances.txt")
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| {
                line.split(";")
                    .map(|float_string| float_string.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
            })
            .collect(),
    );
    let population = Population::random(size_generation, distances.n_units());
    let max_fit = population.get_n_fittest(1, &distances)[0].fitness(&distances);
    let population = (0..n_generations).fold(population, |pop, _| {
        pop.evolve(0.5)
            .get_fittest_population(size_generation, &distances)
    });
    let max_fit_new = population.get_n_fittest(1, &distances)[0].fitness(&distances);
    // Assert after optimizing, the population is fitter then before.
    assert!(max_fit > max_fit_new);
}
