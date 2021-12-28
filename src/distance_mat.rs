use crate::solution::Solution;
#[derive(Debug)]
pub struct DistanceMat {
    distances: Vec<Vec<f64>>,
}

impl DistanceMat {
    pub fn new(distances: Vec<Vec<f64>>) -> Self {
        DistanceMat { distances }
    }
    // TODO: This has to return an `Result`, if there are fewer entries than
    // 2 in `solution` or there is any index in Solution that does not exist in
    // the dist matrix, return a custom error (too few entries / unkown elements)
    pub fn get_distance(&self, solution: &mut Solution) -> f64 {
        solution
            .indexes
            .iter()
            .fold(
                (
                    self.distances[solution.indexes[solution.indexes.len() - 1]]
                        [solution.indexes[0]],
                    None,
                ),
                |(mut loss, last_point): (f64, Option<usize>), current_point| {
                    if let Some(last_point) = last_point {
                        loss += &self.distances[last_point][*current_point];
                    }
                    (loss, Some(*current_point))
                },
            )
            .0
    }
}

#[cfg(test)]
mod test_distance_mat {
    use super::*;
    use crate::test_utils::test_dist_mat;
    #[test]
    fn test_constructor() {
        let dist_mat = DistanceMat::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
        assert_eq!(dist_mat.distances, vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
    }
    #[test]
    fn test_dist_same_node() {
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![0, 0])),
            0.0
        );
    }
    #[test]
    fn test_dist_two_nodes() {
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![0, 1])),
            2.0
        );
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![0, 2])),
            4.0
        );
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![1, 2])),
            6.0
        );
    }
    #[test]
    fn test_dist_three_nodes() {
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![0, 1, 2])),
            6.0
        );
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![0, 2, 1])),
            6.0
        );
    }
    #[test]
    fn test_dist_repeat_visit() {
        assert_eq!(
            test_dist_mat().get_distance(&mut Solution::new(vec![0, 2, 1, 2])),
            10.0
        );
    }
}
