use std::ops::AddAssign;

pub fn kadane<T>(values: &[T]) -> Option<T>
where
    T: Clone + Ord + for<'a> AddAssign<&'a T>,
{
    // verify input validity
    if values.len() == 0 {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = kadane(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]);
        assert_eq!(result, Some(6));
    }
}
