use tracing::info;
use crate::day2::part1::GameStat;

pub fn run_day_2_part2(){
    info!("Running day 1 , Part 1");
    let input = include_str!("./input1.txt");
    info!("Loaded Input File, lines : {}", input.lines().count());
    info!("Loaded Input File, Total String size : {}", input.len());
    let condition = GameStat{
        red: 12,
        green: 13,
        blue: 14,
    };
    info!("Condition ; {:?} ; Sum : {}",&condition , &condition.sum());
    let stats = crate::day2::part1::solve_from_input(input);


    let res = stats.iter().fold(0,|acc,(_game,stat)| {
        acc+stat.power()
    })
    ;
    info!("Result = '{}'", res);
    println!("Result = '{}'", res);
}