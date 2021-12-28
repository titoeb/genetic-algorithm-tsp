use genetic_algo::distance_mat::DistanceMat;
use genetic_algo::solution::Solution;
use std::fs;

#[test]
fn run_evolution() {
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
    let mut _solution = Solution::new(vec![0, 1, 2]);
}
