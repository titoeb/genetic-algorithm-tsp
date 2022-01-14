use genetic_algo::distance_mat::DistanceMat;
use genetic_algo::population::{evolve_population, Population};
use std::fs;
use std::time::Instant;

/// Compute the time in milliseconds that it takes for a genetic algorithm to run.
///
/// # Arguments
///
/// * `n_generations` - How many generations should the algorithm evolve?
/// * `size_generation` - How many individuals should be selected at the end of each
/// evolution step.
/// * `dist_mat` - What is the distance matrix for your TSP.
///
/// ```
pub fn benchmark_population(
    n_generations: usize,
    size_generation: usize,
    dist_mat: &DistanceMat,
) -> u64 {
    // End-to-end test: does the error of the solution get down?
    let before = Instant::now();
    evolve_population(
        Population::random(size_generation, dist_mat.n_units()),
        n_generations,
        size_generation,
        dist_mat,
    );
    let duration = before.elapsed();
    let nanos = duration.subsec_nanos() as u64;
    (1000 * 1000 * 1000 * duration.as_secs() + nanos) / (1000 * 1000)
}
fn main() {
    let distances = DistanceMat::new(
        fs::read_to_string("tests/test-data/distances.txt")
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| {
                line.split(';')
                    .map(|float_string| float_string.parse::<f64>().unwrap())
                    .collect::<Vec<f64>>()
            })
            .collect(),
    );
    for n_generations in (10..=510).step_by(100) {
        for size_generation in (10..=40).step_by(10) {
            let run_time = benchmark_population(n_generations, size_generation, &distances);
            println!(
                "n_generations: {}, size_generation: {}, time: {} ms",
                n_generations, size_generation, run_time
            );
        }
    }
}
