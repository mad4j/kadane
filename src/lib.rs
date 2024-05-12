use std::{cmp::max, ops::Add};

pub fn kadane_00(values: &[i64]) -> i64 {
    let mut best_sum = i64::MIN;
    let mut current_sum = 0;

    for v in values {
        current_sum = max(*v, current_sum + *v);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

pub fn kadane(values: &[i64]) -> i64 {
    let mut best_sum = values[0];
    let mut current_sum = values[0];

    for v in &values[1..] {
        current_sum = max(*v, current_sum + *v);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

pub fn kadane_01<T>(values: &[T]) -> T
where
    T: Copy + Ord + Add<T, Output = T>,
{
    let mut best_sum = values[0];
    let mut current_sum = values[0];

    for v in &values[1..] {
        current_sum = max(*v, current_sum + *v);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

pub fn kadane_02<T>(values: &[T]) -> &T
where
    T: Ord + Add<T, Output = T>,
{
    let mut best_sum = &values[0];
    let mut current_sum = &values[0];

    for v in &values[1..] {

        let t = (*current_sum).clone();

        current_sum = max(v, &t);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = kadane(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);

        let result = kadane_00(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);

        let result = kadane_01(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);
    }
}
