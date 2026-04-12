use std::collections::HashSet;

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
// if I replace rposition with position it becomes 56ms, which means that
// position is faster than rposition
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

// 27ms 2.22MB
pub fn two_sum_4(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, x) in nums.iter().enumerate() {
        let i2 = nums.iter().position(|y| y == &(target - x));
        if i2.is_some_and(|z| z != i) {
            return vec![i as i32, i2.unwrap() as i32];
        }
    }
    Vec::new()
}

pub fn two_sum_5(nums: Vec<i32>, target: i32) -> Vec<i32> {
    todo!()
}

#[cfg(test)]
mod tests {
    use crate::hash_table::two_sum::{two_sum_1, two_sum_2, two_sum_3, two_sum_4, two_sum_5};
    use crate::test_utils::benchmark::{init_benchmark_tracing, run_and_assert_vec_any_order};

    #[derive(Clone)]
    struct TwoSumCase {
        name: &'static str,
        nums: Vec<i32>,
        target: i32,
        expected: Vec<i32>,
    }

    fn full_cases() -> Vec<TwoSumCase> {
        vec![
            TwoSumCase {
                name: "case_1",
                nums: vec![2, 7, 11, 15],
                target: 9,
                expected: vec![0, 1],
            },
            TwoSumCase {
                name: "case_2",
                nums: vec![3, 2, 3],
                target: 6,
                expected: vec![0, 2],
            },
            TwoSumCase {
                name: "case_3",
                nums: vec![3, 2, 4],
                target: 6,
                expected: vec![1, 2],
            },
            TwoSumCase {
                name: "case_4",
                nums: vec![-3, 4, 3, 90],
                target: 0,
                expected: vec![0, 2],
            },
        ]
    }

    fn run_solver_cases(
        solver_name: &str,
        solver: fn(Vec<i32>, i32) -> Vec<i32>,
        cases: Vec<TwoSumCase>,
    ) {
        init_benchmark_tracing();
        for case in cases {
            run_and_assert_vec_any_order(
                &format!("{solver_name}_{}", case.name),
                case.expected,
                || solver(case.nums, case.target),
            );
        }
    }

    #[test]
    #[ignore]
    fn two_sum_1_cases() {
        run_solver_cases("two_sum_1", two_sum_1, full_cases());
    }

    #[test]
    #[ignore]
    fn two_sum_2_cases() {
        run_solver_cases("two_sum_2", two_sum_2, full_cases());
    }

    #[test]
    #[ignore]
    fn two_sum_3_cases() {
        run_solver_cases("two_sum_3", two_sum_3, full_cases());
    }

    #[test]
    #[ignore]
    fn two_sum_4_cases() {
        run_solver_cases("two_sum_4", two_sum_4, full_cases());
    }

    #[test]
    // #[ignore]
    fn two_sum_5_cases() {
        run_solver_cases("two_sum_5", two_sum_5, full_cases());
    }
}
