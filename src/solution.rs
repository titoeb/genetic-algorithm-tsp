use crate::utils::{change_order, get_elem_from_range, remove_elem};
use rand::seq::SliceRandom;

use std::cmp::max;
#[derive(Debug)]
pub struct Solution {
    pub indexes: Vec<usize>,
}
impl Solution {
    pub fn new(indexes: Vec<usize>) -> Self {
        Self { indexes }
    }
    pub fn mutate(self, prob: f32) -> Self {
        Solution {
            indexes: if get_elem_from_range(0.0..1.0) > prob {
                self.indexes
            } else {
                let put_before_idx: usize = get_elem_from_range(0..(self.indexes.len() - 1));
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
    }
}
