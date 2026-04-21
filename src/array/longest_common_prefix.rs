use std::collections::HashSet;

use tracing::info;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut seen = HashSet::<char>::new();
    let mut seen_seen = HashSet::<char>::new();
    let mut common = Vec::<char>::new();
    let mut common_common = Vec::<char>::new();
    let mut unique = Vec::<char>::new();

    for word in strs {
        info!("{word}");
        let chars = word.chars().collect::<Vec<char>>();
        for c in chars.iter() {
            let result = seen.insert(c.to_owned());
            if !result {
                common.push(c.to_owned());
            } else {
                unique.push(c.to_owned());
            }
            // info!("{c}");
            // // f l o w e r
            // // f l o w
            // // f l i g h t
            // let already_seen = seen.iter().find(|x| x == &c);
            // if let Some(v) = already_seen {
            //     info!("Already seen, adding to array: {}", v);
            //     common.push(c.to_owned());
            // } else {
            //     info!("Not seen: {}", c);
            // }
            // seen.push(c.to_owned());
            // info!("Seen: {:?}", seen);
            // info!("Common: {:?}", common);

            // let is_in_common = common.iter().find(|x| x == &c);
            // if let Some(v) = is_in_common {
            //     info!("Already in common: {}", v);
            // } else {
            //     info!("Not in common: {}", c);
            // }
        }
    }

    for cc in &common {
        let result = seen_seen.insert(cc.to_owned());
        if !result {
            common_common.push(cc.to_owned());
        }
    }

    info!("Seen: {:?}", seen);
    info!("Common: {:?}", common);
    info!("Seen seen: {:?}", seen_seen);
    info!("Common common: {:?}", common_common);
    // info!("Unique: {:?}", unique);
    // info!("All characters: {:?}", all_characters);

    // let chars: Vec<Vec<_>> = strs.into_iter().map(|x| x.chars()).collect();

    "prefix".to_string()
}

#[cfg(test)]
mod tests {

    #[test]
    fn longest_common_prefix_1_cases() {
        assert!(true)
    }
}
