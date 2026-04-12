use leetcode::hash_table::two_sum::two_sum_7;
use tracing::info;
use tracing_subscriber::fmt;

fn main() {
    fmt().with_target(false).compact().init();
    info!("-----------------------");
    two_sum_7(vec![2, 7, 11, 15], 9);
    info!("-----------------------");
    two_sum_7(vec![3, 2, 3], 6);
    info!("-----------------------");
    two_sum_7(vec![3, 2, 4], 6);
    info!("-----------------------");
    two_sum_7(vec![-3, 4, 3, 90], 0);
}
