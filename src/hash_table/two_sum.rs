use tracing::info;

// 101ms, 2.37MB
pub fn two_sum_1(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = std::collections::HashSet::<i32>::new();

    nums.iter().enumerate().for_each(|(idx, x)| {
        let pair = nums
            .iter()
            .enumerate()
            .find(|(idx2, y)| *y + x == target && &idx != idx2);
        if let Some((idx2, _)) = pair {
            result.insert(idx as i32);
            result.insert(idx2 as i32);
        }
    });

    result.into_iter().collect()
}

// 56ms, 2.23MB
pub fn two_sum_2(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = std::collections::HashSet::<i32>::new();

    for (idx, num) in nums.iter().enumerate() {
        let pair = target - num;
        let idx2 = nums
            .iter()
            .enumerate()
            .position(|(i, x)| x == &pair && i != idx);
        if let Some(idx2) = idx2 {
            result.insert(idx as i32);
            result.insert(idx2 as i32);
            return result.into_iter().collect();
        }
    }
    result.into_iter().collect()
}

// 86ms, 2.28MB
pub fn two_sum_3(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, x) in nums.iter().enumerate() {
        let idx = nums
            .iter()
            .enumerate()
            .rposition(|(i2, y)| i2 != i && *y == target - x);
        if let Some(idx) = idx {
            return Vec::from([i as i32, idx as i32]);
        }
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::hash_table::two_sum::{two_sum_1, two_sum_2, two_sum_3};
    use crate::test_utils::benchmark::{init_benchmark_tracing, run_with_metrics};

    fn assert_indices_any_order(mut actual: Vec<i32>, mut expected: Vec<i32>) {
        actual.sort_unstable();
        expected.sort_unstable();
        assert_eq!(actual, expected);
    }

    #[test]
    // #[ignore]
    fn two_sum_1_test_1() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_1_test_1", || two_sum_1(vec![2, 7, 11, 15], 9));
        assert_indices_any_order(result, vec![0, 1]);
    }
    #[test]
    // #[ignore]
    fn two_sum_1_test_2() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_1_test_2", || two_sum_1(vec![3, 2, 3], 6));
        assert_indices_any_order(result, vec![0, 2]);
    }
    #[test]
    // #[ignore]
    fn two_sum_1_test_3() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_1_test_3", || two_sum_1(vec![3, 2, 4], 6));
        assert_indices_any_order(result, vec![1, 2]);
    }

    #[test]
    // #[ignore]
    fn two_sum_2_test_1() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_2_test_1", || two_sum_2(vec![2, 7, 11, 15], 9));
        assert_indices_any_order(result, vec![0, 1]);
    }
    #[test]
    // #[ignore]
    fn two_sum_2_test_2() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_2_test_2", || two_sum_2(vec![3, 2, 3], 6));
        assert_indices_any_order(result, vec![0, 2]);
    }
    #[test]
    // #[ignore]
    fn two_sum_2_test_3() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_2_test_3", || two_sum_2(vec![3, 2, 4], 6));
        assert_indices_any_order(result, vec![1, 2]);
    }

    #[test]
    // #[ignore]
    fn two_sum_3_test_1() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_3_test_1", || two_sum_3(vec![2, 7, 11, 15], 9));
        info!("{result:?}");
        assert_indices_any_order(result, vec![0, 1]);
    }
    #[test]
    // #[ignore]
    fn two_sum_3_test_2() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_3_test_2", || two_sum_3(vec![3, 2, 3], 6));
        info!("{result:?}");
        assert_indices_any_order(result, vec![0, 2]);
    }
    #[test]
    // #[ignore]
    fn two_sum_3_test_3() {
        init_benchmark_tracing();
        let (result, _) = run_with_metrics("two_sum_3_test_3", || two_sum_3(vec![3, 2, 4], 6));
        info!("{result:?}");
        assert_indices_any_order(result, vec![1, 2]);
    }
}
