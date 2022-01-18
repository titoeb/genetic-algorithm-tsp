use crate::solution::Solution;
use crate::subsequence::Subsequence;
use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::cmp::max;
use std::cmp::Ordering;
use std::ops::Range;

pub fn get_random_elem_from_range<T>(range: Range<T>) -> T
where
    T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform,
{
    if !range.is_empty() {
        rand::thread_rng().gen_range::<T, Range<T>>(range)
    } else {
        range.start
    }
}
pub fn change_order(data: &[usize], put_before_idx: usize, move_idx: usize) -> Vec<usize> {
    let mut new_data = data.to_owned();
    if put_before_idx != move_idx {
        let move_item = data[move_idx];
        new_data.remove(move_idx);
        let reset_index = (move_idx < put_before_idx) as usize;
        new_data.insert(
            max(put_before_idx, reset_index as usize) - reset_index as usize,
            move_item,
        );
    }
    new_data
}

pub fn remove_elem(mut data: Vec<usize>, elem_idx: &usize) -> Vec<usize> {
    data.remove(*elem_idx);
    data
}

pub fn ordered_crossover(
    parent_a: &Solution,
    parent_b: &Solution,
    subsequence: Subsequence,
) -> Solution {
    let mut child: Vec<usize> = Vec::new();
    let mapped_selection = subsequence.get_values_in(&parent_a.indexes).unwrap();
    // First push elements in subsequence of receiver, that are not in subsequence of donor.
    for elem in subsequence.get_values_in(&parent_b.indexes).unwrap() {
        if !is_in(*elem, mapped_selection) {
            child.push(*elem);
        }
    }
    // Push elements in subsequence of donor.
    for elem in mapped_selection {
        child.push(*elem);
    }
    // Push element after subsequence from receiver, that are not in subsequence of donor.
    for elem in subsequence.get_values_after(&parent_b.indexes).unwrap() {
        if !is_in(*elem, mapped_selection) {
            child.push(*elem);
        }
    }
    // Push element before subsequence from receiver, that are not in subsequence of donor.
    for elem in subsequence.get_values_before(&parent_b.indexes).unwrap() {
        if !is_in(*elem, mapped_selection) {
            child.push(*elem);
        }
    }
    Solution { indexes: child }
}

pub fn is_in(value: usize, elements: &[usize]) -> bool {
    for elem in elements {
        if value == *elem {
            return true;
        }
    }
    false
}

pub fn random_permutation(vec: &[usize]) -> Vec<usize> {
    let mut this_vec: Vec<usize> = vec.to_vec();
    this_vec.shuffle(&mut thread_rng());
    this_vec
}

pub fn argsort<T: PartialOrd>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by(|a_idx, b_idx| {
        data[*a_idx]
            .partial_cmp(&data[*b_idx])
            .unwrap_or(Ordering::Less)
    });
    indices
}

#[cfg(test)]
mod tests {
    use super::*;
    mod get_elem_from_range {
        use super::*;
        #[test]
        fn sample_int_range() {
            get_elem_from_range(0..10);
        }
        #[test]
        fn sample_float_range() {
            get_elem_from_range(0.0..1.0);
        }
        #[test]
        fn sample_empty_range() {
            assert_eq!(get_elem_from_range(0..0), 0);
        }
    }
    mod test_remove_elem {
        use super::*;
        #[test]
        fn remove_first() {
            assert_eq!(remove_elem(vec![1, 2, 3, 4], &0), vec![2, 3, 4]);
        }
        #[test]
        fn remove_last() {
            assert_eq!(remove_elem(vec![1, 2, 3, 4], &3), vec![1, 2, 3]);
        }
        #[test]
        fn remove_middle() {
            assert_eq!(remove_elem(vec![1, 2, 3, 4], &2), vec![1, 2, 4]);
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
    }
    mod test_change_elem {
        use super::*;
        #[test]
        fn put_before_first() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 0, 1), vec![2, 1, 3, 4]);
        }
        #[test]
        fn put_last_before_first() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 0, 3), vec![4, 1, 2, 3]);
        }
        #[test]
        fn put_first_before_second() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 1, 0), vec![1, 2, 3, 4]);
        }
        #[test]
        fn put_before_second() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 1, 2), vec![1, 3, 2, 4]);
        }
        #[test]
        fn put_last_before_second() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 1, 3), vec![1, 4, 2, 3]);
        }
        #[test]
        fn put_first_before_last() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 3, 0), vec![2, 3, 1, 4]);
        }
        #[test]
        fn put_fourth_before_fourth() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 3, 3), vec![1, 2, 3, 4]);
        }
        #[test]
        fn put_first_before_first() {
            assert_eq!(change_order(&vec![1, 2, 3, 4], 3, 3), vec![1, 2, 3, 4]);
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
    mod test_ordered_crossover {
        use super::*;
        #[test]
        fn test_from_paper() {
            // test taken from example in https://citeseerx.ist.psu.edu/viewdoc/download?doi=10.1.1.50.1898&rep=rep1&type=pdf.
            assert_eq!(
                ordered_crossover(
                    &Solution {
                        indexes: vec![9, 8, 4, 5, 6, 7, 1, 3, 2]
                    },
                    &Solution {
                        indexes: vec![8, 7, 1, 2, 3, 0, 9, 5, 4]
                    },
                    Subsequence {
                        start_index: 3,
                        length: 3
                    }
                )
                .indexes,
                vec![2, 3, 0, 5, 6, 7, 9, 4, 8, 1]
            )
        }
        #[test]
        fn simple_test() {
            assert_eq!(
                ordered_crossover(
                    &Solution {
                        indexes: vec![3, 2, 0, 1]
                    },
                    &Solution {
                        indexes: vec![1, 2, 3, 0]
                    },
                    Subsequence {
                        start_index: 1,
                        length: 2
                    }
                )
                .indexes,
                vec![3, 2, 0, 1]
            )
        }
        #[test]
        fn only_a() {
            assert_eq!(
                ordered_crossover(
                    &Solution {
                        indexes: vec![3, 2, 0, 1]
                    },
                    &Solution {
                        indexes: vec![1, 2, 3, 0]
                    },
                    Subsequence {
                        start_index: 0,
                        length: 4
                    }
                )
                .indexes,
                vec![3, 2, 0, 1]
            )
        }
        #[test]
        fn only_b() {
            assert_eq!(
                ordered_crossover(
                    &Solution {
                        indexes: vec![3, 2, 0, 1]
                    },
                    &Solution {
                        indexes: vec![1, 2, 3, 0]
                    },
                    Subsequence {
                        start_index: 0,
                        length: 0
                    }
                )
                .indexes,
                vec![1, 2, 3, 0]
            )
        }
        #[test]
        fn test_from_online_example() {
            // Example taken from
            // https://www.rubicite.com/Tutorials/GeneticAlgorithms/CrossoverOperators/Order1CrossoverOperator.aspx
            assert_eq!(
                ordered_crossover(
                    &Solution {
                        indexes: vec![8, 4, 7, 3, 6, 2, 5, 1, 9, 0]
                    },
                    &Solution {
                        indexes: vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]
                    },
                    Subsequence {
                        start_index: 3,
                        length: 5
                    }
                )
                .indexes,
                vec![4, 7, 3, 6, 2, 5, 1, 8, 9, 0]
            )
        }
        #[test]
        fn larger_examples() {
            assert_eq!(
                ordered_crossover(
                    &Solution {
                        indexes: vec![0, 12, 7, 3, 9, 8, 11, 5, 13, 1, 4, 6, 10, 15, 2, 14],
                    },
                    &Solution {
                        indexes: vec![7, 10, 15, 12, 2, 9, 5, 3, 1, 6, 4, 13, 14, 11, 8, 0],
                    },
                    Subsequence {
                        start_index: 13,
                        length: 2
                    }
                )
                .indexes,
                vec![11, 8, 15, 2, 0, 7, 10, 12, 9, 5, 3, 1, 6, 4, 13, 14,]
            )
        }
    }
    mod test_is_in {
        use super::*;
        #[test]
        fn not_in() {
            assert_eq!(is_in(0, &[1, 2, 3]), false)
        }
        #[test]
        fn not_in_empty_sequence() {
            assert_eq!(is_in(0, &Vec::<usize>::new()), false)
        }
        #[test]
        fn value_is_in() {
            assert_eq!(is_in(0, &[1, 0, 3]), true)
        }
        #[test]
        fn value_is_in_duplicated() {
            assert_eq!(is_in(0, &[0, 1, 0, 3]), true)
        }
    }
    mod test_random_permutation {
        use super::*;
        use crate::test_utils::valid_permutation;
        #[test]
        #[test]
        #[test]
        #[test]
        fn simple_test() {
            let main_vec = (0..10).collect::<Vec<usize>>();
            valid_permutation(&main_vec, &random_permutation(&main_vec));
        }
    }
    mod test_argsort {
        use super::*;
        #[test]
        fn four_floats() {
            assert_eq!(argsort(&vec![1.0, 5.0, 3.0, 6.0]), vec![0, 2, 1, 3]);
        }
        #[test]
        fn thirteen_floats() {
            assert_eq!(
                argsort(&vec![
                    13.0, 14.0, 12.0, 10.0, 22.0, 6.0, 16.0, 24.0, 18.0, 23.0, 15.0, 11.0, 17.0
                ]),
                vec![5, 3, 11, 2, 0, 1, 10, 6, 12, 8, 4, 9, 7]
            );
        }

        #[test]
        fn five_isize() {
            assert_eq!(argsort(&vec![2, 5, 3, 4, 1, 6]), vec![4, 0, 2, 3, 1, 5]);
        }
    }
}
