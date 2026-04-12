// First try - brute force approach without optimization
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
// is position is faster than rposition?
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

// Can we change our array somehow so the search become faster?
// 3ms 2.29MB
pub fn two_sum_5(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut arr = nums.clone();
    for _ in nums.iter() {
        let last = arr.pop();
        if let Some(last) = last {
            let result = target - last;
            let i1 = nums.iter().rposition(|x| x == &last);
            let i2 = arr.iter().position(|y| y == &result);
            if let (Some(i1), Some(i2)) = (i1, i2) {
                return vec![i2 as i32, i1 as i32];
            }
        }
    }
    Vec::new()
}

// Without changing the array, can use additional space to speed up the search?
// 65ms 2.54MB
pub fn two_sum_6(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: std::collections::HashMap<_, _> = nums.iter().enumerate().collect();
    for (i1, x) in nums.iter().enumerate() {
        map.remove(&i1);
        let result = target - x;
        if let Some((i2, _)) = &map.iter().find(|(_, v)| ***v == result) {
            return vec![i1 as i32, **i2 as i32];
        }
    }
    Vec::new()
}

// 3ms 3.2MB
pub fn two_sum_7(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let map = nums.iter().enumerate().fold(
        std::collections::HashMap::<i32, Vec<i32>>::new(),
        |mut map, (idx, &val)| {
            map.entry(val).or_default().push(idx as i32);
            map
        },
    );

    let mut answer = std::collections::HashSet::<i32>::new();

    for (i, x) in map.iter() {
        let result = target - i;
        if result == *i && x.len().eq(&1) {
            continue;
        };
        let index = map.get(&result);
        if let Some(index) = index {
            answer.extend(index);
        }
    }
    answer.into_iter().collect()
}

// 1ms 3.11MB
// Sometimes: 0ms 3.02MB
//
// This solution thinks in terms of pairs: build a complete map of everything,
// then search for matching pairs. This is natural but leads to complexity —
// you have to handle edge cases like duplicate values, and you process the
// whole array before finding anything.
pub fn two_sum_8(nums: Vec<i32>, target: i32) -> Vec<i32> {
    if nums.len() == 2 {
        return [0, 1].to_vec();
    }

    let map = nums.iter().enumerate().fold(
        std::collections::HashMap::<i32, Vec<i32>>::new(),
        |mut map, (idx, &val)| {
            map.entry(val).or_default().push(idx as i32);
            map
        },
    );

    let result: Vec<i32> = map
        .iter()
        .filter_map(|(value, keys)| {
            let complement = target - value;
            // Skip the case where the value pairs with itself but only has one index
            if complement == *value && keys.len() == 1 {
                return None;
            }
            map.get(&complement)
        })
        .flat_map(|indices| indices.iter().copied())
        .collect();

    result
}

// 0ms 2.56MB
//
// Whenever you build a map over a whole collection before querying it,
// ask yourself: "Could I merge the build and query into one pass?"
//
// The optimal solution asks a different question at each step:
// "Have I already seen the number I need?"
//
// Think of it this way: you're walking through the array left to right.
// At any position i, the map contains only the elements you've already passed.
// You're not comparing against the whole array — you're asking: "among everything
// I've seen so far, does my complement exist?"
pub fn two_sum_9(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = std::collections::HashMap::<i32, usize>::new();

    for (i, x) in nums.iter().enumerate() {
        let result = target - x;

        let y = seen.get(&result);

        if let Some(y) = y {
            return vec![*y as i32, i as i32];
        }

        seen.insert(*x, i);
    }

    Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::hash_table::two_sum::{
        two_sum_1, two_sum_2, two_sum_3, two_sum_4, two_sum_5, two_sum_6, two_sum_7, two_sum_8,
        two_sum_9,
    };
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
    #[ignore]
    fn two_sum_5_cases() {
        run_solver_cases("two_sum_5", two_sum_5, full_cases());
    }

    #[test]
    #[ignore]
    fn two_sum_6_cases() {
        run_solver_cases("two_sum_6", two_sum_6, full_cases());
    }

    #[test]
    #[ignore]
    fn two_sum_7_cases() {
        run_solver_cases("two_sum_7", two_sum_7, full_cases());
    }

    #[test]
    #[ignore]
    fn two_sum_8_cases() {
        run_solver_cases("two_sum_8", two_sum_8, full_cases());
    }

    #[test]
    // #[ignore]
    fn two_sum_9_cases() {
        run_solver_cases("two_sum_9", two_sum_9, full_cases());
    }
}
