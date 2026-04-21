use tracing::info;

pub fn longest_common_prefix_1(strs: Vec<String>) -> String {
    let mut seen = std::collections::HashSet::<char>::new();
    let mut seen_seen = std::collections::HashSet::<char>::new();
    let mut common = Vec::<char>::new();
    let mut common_common = Vec::<char>::new();

    for word in strs {
        let chars = word.chars().collect::<Vec<char>>();
        for c in chars.iter() {
            let result = seen.insert(c.to_owned());
            if !result {
                common.push(c.to_owned());
            }
        }
    }

    for cc in &common {
        let result = seen_seen.insert(cc.to_owned());
        if !result {
            common_common.push(cc.to_owned());
        }
    }

    info!("{common_common:?}");

    String::from_iter(common_common)
}

#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::{
        array::longest_common_prefix::longest_common_prefix_1,
        test_utils::benchmark::init_benchmark_tracing,
    };

    struct Case {
        input: Vec<String>,
        output: String,
    }

    fn full_cases() -> Vec<Case> {
        vec![
            Case {
                input: vec![
                    "flower".to_string(),
                    "flow".to_string(),
                    "flight".to_string(),
                ],
                output: "fl".to_string(),
            },
            Case {
                input: vec!["dog".to_string(), "racecar".to_string(), "car".to_string()],
                output: String::new(),
            },
        ]
    }

    fn run_solver_cases(solver_name: &str, solver: fn(Vec<String>) -> String, cases: Vec<Case>) {
        init_benchmark_tracing();
        info!("{solver_name}");
        for case in cases {
            assert_eq!(solver(case.input), case.output);
        }
    }

    #[test]
    fn longest_common_prefix_1_cases() {
        run_solver_cases("solver 1", longest_common_prefix_1, full_cases());
    }
}
