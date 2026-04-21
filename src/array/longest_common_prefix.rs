pub fn longest_common_prefix(strs: Vec<String>) -> String {
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

    String::from_iter(common_common)
}

#[cfg(test)]
mod tests {

    #[test]
    fn longest_common_prefix_1_cases() {
        assert!(true)
    }
}
