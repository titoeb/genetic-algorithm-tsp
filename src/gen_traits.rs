use crate::solution::Solution;

/// Data used for the computation of the costs in genetic algorithms.
pub trait CostData {
    /// Compute the costs (reverse fitness) of an individual.
    ///
    /// # Arguments
    ///
    /// * `solution` - The individual that should be tested.
    ///
    fn compute_cost(&self, solution: &Solution) -> f64;
}
