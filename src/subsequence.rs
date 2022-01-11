use crate::utils::get_elem_from_range;
#[derive(Debug)]
pub struct Subsequence {
    pub start_index: usize,
    pub length: usize,
}

impl Subsequence {
    pub fn new(start_index: usize, length: usize) -> Self {
        Subsequence {
            start_index,
            length,
        }
    }
    pub fn random_subsequence(max: usize) -> Self {
        let start_index = get_elem_from_range(0..(max - 2));
        Subsequence {
            start_index,
            length: get_elem_from_range(1..(max - start_index - 1)),
        }
    }
    pub fn get_values_in<'a>(&self, sequence: &'a [usize]) -> Option<&'a [usize]> {
        if self.start_index + self.length <= sequence.len() {
            Some(&sequence[self.start_index..(self.start_index + self.length)])
        } else {
            None
        }
    }
    pub fn get_values_before<'a>(&self, sequence: &'a [usize]) -> Option<&'a [usize]> {
        if self.start_index <= sequence.len() {
            Some(&sequence[..self.start_index])
        } else {
            None
        }
    }
    pub fn get_values_after<'a>(&self, sequence: &'a [usize]) -> Option<&'a [usize]> {
        if self.start_index + self.length <= sequence.len() {
            Some(&sequence[(self.start_index + self.length)..])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod test_random_subsequence {
        use super::*;
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_max_10() {
            let max_value = 10;
            let random_subsequence = Subsequence::random_subsequence(max_value);
            assert!(random_subsequence.start_index < max_value);
            assert!(random_subsequence.length < max_value - random_subsequence.start_index);
            assert!(random_subsequence.start_index + random_subsequence.length < max_value);
        }
        #[test]
        #[test]
        #[test]
        #[test]
        fn test_max_2() {
            let max_value = 2;
            let random_subsequence = Subsequence::random_subsequence(max_value);
            assert!(random_subsequence.start_index < max_value);
            assert!(random_subsequence.length < max_value - random_subsequence.start_index);
            assert!(random_subsequence.start_index + random_subsequence.length < max_value);
        }
    }
    mod test_get_values_in_subsequence {
        use super::*;
        #[test]
        fn partial_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 3,
                length: 4,
            };
            assert_eq!(subsequence.get_values_in(&sequence), Some(&sequence[3..=6]))
        }
        #[test]
        fn full_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 0,
                length: 10,
            };
            assert_eq!(subsequence.get_values_in(&sequence), Some(&sequence[0..]))
        }
        #[test]
        fn too_short() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 5,
                length: 12,
            };
            assert_eq!(subsequence.get_values_in(&sequence), None)
        }
    }
    mod test_get_values_before_subsequence {
        use super::*;
        #[test]
        fn partial_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 3,
                length: 4,
            };
            assert_eq!(
                subsequence.get_values_before(&sequence),
                Some(&sequence[0..3])
            )
        }
        #[test]
        fn full_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 0,
                length: 10,
            };
            assert_eq!(
                subsequence.get_values_before(&sequence),
                Some(&Vec::<usize>::new()[0..])
            )
        }
        #[test]
        fn too_short_for_sequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 5,
                length: 12,
            };
            assert_eq!(
                subsequence.get_values_before(&sequence),
                Some(&sequence[0..5])
            )
        }
        #[test]
        fn too_short_for_before_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 11,
                length: 12,
            };
            assert_eq!(subsequence.get_values_before(&sequence), None)
        }
    }
    mod test_get_values_after_subsequence {
        use super::*;
        #[test]
        fn partial_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 3,
                length: 4,
            };
            assert_eq!(
                subsequence.get_values_after(&sequence),
                Some(&sequence[7..10])
            )
        }
        #[test]
        fn full_subsequence() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 0,
                length: 10,
            };
            assert_eq!(
                subsequence.get_values_after(&sequence),
                Some(&Vec::<usize>::new()[0..])
            )
        }
        #[test]
        fn too_short() {
            let sequence: Vec<usize> = (0..10).collect();
            let subsequence = Subsequence {
                start_index: 5,
                length: 12,
            };
            assert_eq!(subsequence.get_values_after(&sequence), None)
        }
    }
}
