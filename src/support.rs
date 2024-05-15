use std::{
    cmp::max,
    ops::{Add, AddAssign},
};

// straight implementation
pub fn kadane_00(values: &[i64]) -> i64 {
    let mut best_sum = i64::MIN;
    let mut current_sum = 0;

    for &v in values {
        current_sum = max(v, current_sum + v);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

// straight implementation without use of max function
pub fn kadane_00a(values: &[i64]) -> i64 {
    let mut best_sum = i64::MIN;
    let mut current_sum = 0;

    for &v in values {
        current_sum += v;

        if v > current_sum {
            current_sum = v;
        }

        if current_sum > best_sum {
            best_sum = current_sum;
        }
    }

    best_sum
}

// straight implementation without unused initialization
pub fn kadane_01(values: &[i64]) -> i64 {
    let mut best_sum = values[0];
    let mut current_sum = values[0];

    for &v in &values[1..] {
        current_sum = max(v, current_sum + v);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

// straight implementation without unused initialization
pub fn kadane_01a(values: &[i64]) -> i64 {
    let mut best_sum = values[0];
    let mut current_sum = values[0];

    for &v in &values[1..] {
        current_sum += v;

        if v > current_sum {
            current_sum = v;
        }

        if current_sum > best_sum {
            best_sum = current_sum;
        }
    }

    best_sum
}

pub fn kadane_02<T>(values: &[T]) -> T
where
    T: Copy + Ord + Add<T, Output = T>,
{
    let mut best_sum = values[0];
    let mut current_sum = values[0];

    for &v in &values[1..] {
        current_sum = max(v, current_sum + v);
        best_sum = max(best_sum, current_sum);
    }

    best_sum
}

pub fn kadane_03<T>(values: &[T]) -> T
where
    T: Clone + Ord + for<'a> Add<&'a T, Output = T>,
{
    let mut best_sum = values[0].clone();
    let mut current_sum = values[0].clone();

    for v in &values[1..] {
        current_sum = current_sum + v;

        if *v > current_sum {
            current_sum = v.clone();
        }

        if current_sum > best_sum {
            best_sum = current_sum.clone();
        }
    }

    best_sum
}

pub fn kadane_04<T>(values: &[T]) -> Option<T>
where
    T: Clone + Ord + for<'a> AddAssign<&'a T>,
{
    if values.len() == 0 {
        return None;
    }

    let mut best_sum = values[0].clone();
    let mut current_sum = values[0].clone();

    values.iter().skip(1).for_each(|v| {
        //for v in &values[1..] {
        current_sum += v;

        if *v > current_sum {
            current_sum = v.clone();
        }

        if current_sum > best_sum {
            best_sum = current_sum.clone();
        }
    });
    Some(best_sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = kadane_00(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);

        let result = kadane_01(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);

        let result = kadane_02(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);

        let result = kadane_03(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, 6);

        let result = kadane_04(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, Some(6));
    }
}
