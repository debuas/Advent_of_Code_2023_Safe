use crate::day1::part1::run_day1_part_1;

use tracing_subscriber;
use crate::day1::part2::run_day1_part_2;
use crate::day2::part1::run_day_2_part1;
use crate::day2::part2::run_day_2_part2;

mod day1;
mod day2;

fn main() {
    tracing_subscriber::fmt::init();

    run_day1_part_1();
    run_day1_part_2();
    run_day_2_part1();
    run_day_2_part2()
}
