use std::collections::HashSet;

// First try
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = HashSet::<i32>::new();

    nums.iter().enumerate().for_each(|(idx, x)| {
        let pair = nums
            .iter()
            .enumerate()
            .find(|(idx2, y)| *y + x == target && &idx != idx2);
        if let Some((idx2, y)) = pair {
            result.insert(idx as i32);
            result.insert(idx2 as i32);
            println!("result: {} + {:?} = {}", x, y, target);
        }
    });

    println!("{:#?}", result);
    result.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use crate::hash_table::two_sum::two_sum;

    #[test]
    fn two_sum_test_1() {
        two_sum(vec![2, 7, 11, 15], 9);
    }
    #[test]
    fn two_sum_test_2() {
        two_sum(vec![3, 2, 3], 6);
    }
    #[test]
    fn two_sum_test_3() {
        two_sum(vec![3, 2, 4], 6);
    }
}
