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

/// Individual used in the genetic algorithm
pub trait Individual {
    /// Randomly changes the order of two nodes in the solution
    ///
    /// # Arguments
    ///
    /// * `prob` - The probability with which the indexes will be changed
    fn mutate(self, prob: f32) -> Self;
    /// Crossover this invidual with another individual to create a new individual. Currently
    /// uses the `ordered_crossover` algorithm.
    ///
    /// # Arguments
    ///
    /// * `other` - The other individual you would like to use in the crossover individual.
    ///
    fn crossover(&self, other: &Self) -> Self;
    /// Compute how much distance the individual implies with its order of nodes
    /// and the distance matrix.
    ///
    /// # Arguments
    ///
    /// * `distance_matrix` - Distance Matrix that determines the length of the proposed
    /// solution
    ///
    fn fitness(&self, cost_data: &impl CostData) -> f64;
    // {
    //     cost_data.compute_cost(self)
    // }
}
