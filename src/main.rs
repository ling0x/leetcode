use leetcode::{
    array::longest_common_prefix::longest_common_prefix, hash_table::two_sum::two_sum_9,
};
use tracing::info;
use tracing_subscriber::fmt;

fn main() {
    fmt().with_target(false).compact().init();
    info!("-----------------------");
    longest_common_prefix(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
}
