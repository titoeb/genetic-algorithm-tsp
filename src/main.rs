use genetic_algo::distance_mat::DistanceMat;
use genetic_algo::population::benchmark_population;
use std::fs;

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
