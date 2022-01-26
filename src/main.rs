use genetic_algo::distance_mat::DistanceMat;
use genetic_algo::routes::benchmark_population;
use std::fs;

fn main() {
    // Read-in test distance matrix from `tests/test-data` folder.
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
            let (run_time, minimal_loss) =
                benchmark_population(n_generations, size_generation, &distances, 0);
            println!(
                "n_generations: {}, size_generation: {}, time: {} ms, minimal loss: {}",
                n_generations, size_generation, run_time, minimal_loss
            );
        }
    }
    println!("Running multi-threaded computation!");
    let n_jobs = 8;
    for n_generations in (10..=1100).step_by(100) {
        for size_generation in (10..=80).step_by(10) {
            let (run_time, minimal_loss) =
                benchmark_population(n_generations, size_generation, &distances, n_jobs);
            println!(
                "n_generations: {}, size_generation: {}, time: {} ms, minimal loss: {}, n_jobs: {}",
                n_generations, size_generation, run_time, minimal_loss, n_jobs
            );
        }
    }
}
