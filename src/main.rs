use crate::day1::part1::run_day1_part_1;


use crate::day1::part2::run_day1_part_2;
use crate::day2::part1::run_day_2_part1;
use crate::day2::part2::run_day_2_part2;
use crate::day3::day3::{run_day_3_part_1, run_day_3_part_2};
use crate::day4::day4::{run_day_4_part_1, run_day_4_part_2};
use crate::day5::day5::{run_day_5_part1, run_day_5_part2};
use crate::day6::day6::{run_day_6_part_1, run_day_6_part_2};
use crate::day7::day7::{run_day_7_part_1, run_day_7_part_2};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    tracing_subscriber::fmt::init();

    run_day1_part_1();
    run_day1_part_2();
    run_day_2_part1();
    run_day_2_part2();
    run_day_3_part_1();
    run_day_3_part_2();
    run_day_4_part_1();
    run_day_4_part_2();
    run_day_5_part1();
    run_day_5_part2();
    run_day_6_part_1();
    run_day_6_part_2();
    run_day_7_part_1();
    run_day_7_part_2();
}
