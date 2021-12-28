use crate::distance_mat::DistanceMat;

pub fn test_dist_mat() -> DistanceMat {
    DistanceMat::new(vec![
        vec![0.0, 1.0, 2.0],
        vec![1.0, 0.0, 3.0],
        vec![2.0, 3.0, 0.0],
    ])
}
