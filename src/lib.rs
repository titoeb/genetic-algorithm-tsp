use rand::seq::SliceRandom;
use rand::Rng;
use std::cmp::max;
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

#[derive(Debug)]
pub struct Solution {
    indexes: Vec<usize>,
}

fn change_order(data: &[usize], put_before_idx: usize, move_idx: usize) -> Vec<usize> {
    let mut new_data = data.to_owned();
    let move_item = data[move_idx];
    new_data.remove(move_idx);
    new_data.insert(max(put_before_idx, 1) - 1, move_item);
    new_data
}

fn remove_elem(mut data: Vec<usize>, elem_idx: &usize) -> Vec<usize> {
    data.remove(*elem_idx);
    data
}

impl Solution {
    pub fn new(indexes: Vec<usize>) -> Self {
        Self { indexes }
    }
    pub fn mutate(self, prob: f32) -> Self {
        Solution {
            indexes: if rand::thread_rng().gen_range(0.0..1.0) > prob {
                self.indexes
            } else {
                let put_before_idx: usize =
                    rand::thread_rng().gen_range(0..(self.indexes.len() - 1));
                change_order(
                    &self.indexes,
                    put_before_idx,
                    *remove_elem(
                        remove_elem(
                            (0..(self.indexes.len() - 1)).collect::<Vec<usize>>(),
                            &put_before_idx,
                        ),
                        &(max(put_before_idx, 1) - 1),
                    )
                    .choose(&mut rand::thread_rng())
                    .unwrap(),
                )
            },
        }
    }
    pub fn crossover(self, _other: &Solution) -> Self {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_solution {
        use super::*;
        #[test]
        fn test_constructor() {
            let solution = Solution::new(vec![1, 2, 3, 4]);
            assert_eq!(solution.indexes, vec![1, 2, 3, 4])
        }
        #[test]
        fn test_mutuate_no_prob() {
            assert_eq!(
                Solution::new(vec![1, 2, 3, 4]).mutate(0.0).indexes,
                vec![1, 2, 3, 4]
            )
        }
        // Run the following test five times.
        #[test]
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_mutuate_100_prob() {
            assert_ne!(
                Solution::new(vec![1, 2, 3, 4]).mutate(1.0).indexes,
                vec![1, 2, 3, 4]
            )
        }
        #[test]
        fn test_remove_elem_first() {
            assert_eq!(remove_elem(vec![1, 2, 3], &0), vec![2, 3])
        }
        #[test]
        fn test_remove_elem_middle() {
            assert_eq!(remove_elem(vec![1, 2, 3], &1), vec![1, 3])
        }
        #[test]
        fn test_remove_elem_last() {
            assert_eq!(remove_elem(vec![1, 2, 3], &2), vec![1, 2])
        }
        #[test]
        fn test_change_order_move_first() {
            assert_eq!(change_order(&vec![1, 2, 3], 1, 0), vec![1, 2, 3])
        }
        #[test]
        fn test_change_order_move_middle() {
            assert_eq!(change_order(&vec![1, 2, 3], 0, 1), vec![2, 1, 3])
        }

        #[test]
        fn test_change_order_move_last() {
            assert_eq!(change_order(&vec![1, 2, 3], 0, 2), vec![3, 1, 2])
        }
        #[test]
        fn test_change_order_move_first_before_last() {
            assert_eq!(change_order(&vec![1, 2, 3], 2, 0), vec![2, 1, 3])
        }
        #[test]
        fn test_change_order_move_middle_before_last() {
            assert_eq!(change_order(&vec![1, 2, 3], 2, 1), vec![1, 2, 3])
        }
    }
    mod test_distance_mat {
        use super::*;
        #[test]
        fn test_constructor() {
            let dist_mat = DistanceMat::new(vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
            assert_eq!(dist_mat.distances, vec![vec![0.0, 1.0], vec![1.0, 0.0]]);
        }
        #[test]
        fn test_dist_same_node() {
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![0, 0])),
                0.0
            );
        }
        #[test]
        fn test_dist_two_nodes() {
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![0, 1])),
                2.0
            );
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![0, 2])),
                4.0
            );
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![1, 2])),
                6.0
            );
        }
        #[test]
        fn test_dist_three_nodes() {
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![0, 1, 2])),
                6.0
            );
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![0, 2, 1])),
                6.0
            );
        }
        #[test]
        fn test_dist_repeat_visit() {
            assert_eq!(
                utils::test_dist_mat().get_distance(&mut Solution::new(vec![0, 2, 1, 2])),
                10.0
            );
        }
    }
    mod utils {
        use super::*;
        pub fn test_dist_mat() -> DistanceMat {
            DistanceMat::new(vec![
                vec![0.0, 1.0, 2.0],
                vec![1.0, 0.0, 3.0],
                vec![2.0, 3.0, 0.0],
            ])
        }
    }
}
