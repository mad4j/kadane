//! Kadane algorithm implementation
//! Daniele Olmisani <daniele.olmisani@gmail.com>
//!
//! see LICENSE file

use std::ops::AddAssign;

/// Implements Kadane algorithm for maximum subarray problem
/// Complexity: O(n) time, O(1) space
///
/// # Examples
/// ```
/// use kadane::kadane;
/// assert_eq!(kadane::max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), Some(6));
/// ```
/// 
/// # Returns
/// * the sum of max subarray values. 
/// * `None` in case of zero-length array.
///
pub fn max_subarray_sum<T>(values: &[T]) -> Option<T>
where
    T: Clone + PartialOrd + for<'a> AddAssign<&'a T>,
{
    // verify input validity
    if values.is_empty() {
        return None;
    }

    // initialize partial sums
    let mut best_sum = values[0].clone();
    let mut current_sum = values[0].clone();

    // use iterator instead of range for better performances
    // for v in &values[1..] {
    values.iter().skip(1).for_each(|v| {
        // update current sum
        current_sum += v;

        // last element alone better of current sum
        if *v > current_sum {
            current_sum = v.clone();
        }

        // current sum better of last better sum
        if current_sum > best_sum {
            best_sum = current_sum.clone();
        }
    });

    // return better sum
    Some(best_sum)
}

/// Implements Kadane algorithm for maximum subarray problem.
/// Complexity: O(n) time, O(1) space.
///
/// # Examples
/// ```
/// use kadane::kadane;
/// assert_eq!(kadane::max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]).unwrap(), [4, -1, 2, 1]);
/// ```
/// 
/// # Returns
/// * the slice containing the subarray with max value 
/// * `None` in case of zero-length array
///
pub fn max_subarray<T>(values: &[T]) -> Option<&[T]>
where
    T: Clone + PartialOrd + for<'a> AddAssign<&'a T>,
{
    // verify input validity
    if values.is_empty() {
        return None;
    }

    // initialize partial sums
    let mut best_sum = values[0].clone();
    let mut current_sum = values[0].clone();

    // best sum boundaries
    let mut best_first_index = 0;
    let mut best_last_index = 1;

    // current sum boundaries
    let mut current_first_index = 0;
    let mut current_last_index = 1;

    // use iterator instead of range for better performances
    values.iter().enumerate().skip(1).for_each(|(i, v)| {
        // update current sum
        current_sum += v;
        current_last_index = i + 1;

        // last element alone better of current sum
        if *v > current_sum {
            current_sum = v.clone();
            current_first_index = i;
        }

        // current sum better of last better sum
        if current_sum > best_sum {
            best_sum = current_sum.clone();
            best_first_index = current_first_index;
            best_last_index = current_last_index;
        }
    });

    // return better sum
    Some(&values[best_first_index..best_last_index])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kadane() {
        let result = max_subarray_sum(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_kadane_seq() {
        let result = max_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result.unwrap(), [4, -1, 2, 1]);
        assert_eq!(result.unwrap().iter().sum::<i64>(), 6);
        assert_eq!(result.unwrap().len(), 4);
    }


    #[test]
    fn test_kadane_float() {
        let result = max_subarray_sum(&[-2.0, 1.0, -3.0, 4.0, -1.0, 2.0, 1.0, -5.0, 4.0]);
        assert_eq!(result, Some(6.0));
    }

    #[test]
    fn test_kadane_seq_float() {
        let result = max_subarray(&[-2.0, 1.0, -3.0, 4.0, -1.0, 2.0, 1.0, -5.0, 4.0]);
        assert_eq!(result.unwrap(), [4.0, -1.0, 2.0, 1.0]);
        assert_eq!(result.unwrap().iter().sum::<f64>(), 6.0);
        assert_eq!(result.unwrap().len(), 4);
    }


    #[test]
    fn test_kadane_vector() {
        let result = max_subarray_sum(&vec![-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_kadane_seq_vector() {
        let binding = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
        let result = max_subarray(&binding);
        assert_eq!(result.unwrap(), vec![4, -1, 2, 1]);
        assert_eq!(result.unwrap().iter().sum::<i64>(), 6);
        assert_eq!(result.unwrap().len(), 4);
    }

}
