use std::ops::AddAssign;

pub fn kadane<T>(values: &[T]) -> Option<T>
where
    T: Clone + Ord + for<'a> AddAssign<&'a T>,
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

pub fn kadane_seq<T>(values: &[T]) -> Option<&[T]>
where
    T: Clone + Ord + for<'a> AddAssign<&'a T>,
{
    // verify input validity
    if values.is_empty() {
        return None;
    }

    // initialize partial sums
    let mut best_sum = values[0].clone();
    let mut current_sum = values[0].clone();

    let mut best_first_index = 0;
    let mut best_last_index = 1;
    let mut current_first_index = 0;
    let mut current_last_index = 1;

    // use iterator instead of range for better performances
    values.iter().enumerate().skip(1).for_each(|(i, v)| {
        
        // update current sum
        current_sum += v;
        current_last_index = i+1;

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
        let result = kadane(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_kadane_seq() {
        let result = kadane_seq(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        println!("{:?}", result);
        assert_eq!(result.unwrap().len(), 4);
    }
}
