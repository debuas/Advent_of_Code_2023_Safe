use crate::day1::part1::run_part_1;

use tracing_subscriber;
use crate::day1::part2::run_part_2;

mod day1;

fn main() {
    tracing_subscriber::fmt::init();

    run_part_1();
    run_part_2();
}
