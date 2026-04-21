use leetcode::{
    array::longest_common_prefix::longest_common_prefix_1, hash_table::two_sum::two_sum_9,
};
use tracing::info;
use tracing_subscriber::fmt;

fn main() {
    fmt().with_target(false).compact().init();
    info!("-----------------------");
    longest_common_prefix_1(vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ]);
}
