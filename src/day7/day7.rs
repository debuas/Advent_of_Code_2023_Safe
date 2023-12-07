use std::str::Lines;
use rayon::iter::ParallelIterator;
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use tracing::{debug, info, instrument};

pub fn run_day_7_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input) ;

    info!("Result :  {:?}" , res);
    println!("Result :  {:?}" , res)
}

pub fn run_day_7_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input) ;


    info!("Result :  {:?}" , res);
    println!("Result :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> Lines<'_> {
    let res = input
        .lines()
;
    debug!("{:?}",res);

    res

}
pub fn from_input_part_2(input : &str ) -> Lines<'_> {
    let res = input
        .lines()

    ;
    debug!("{:?}",res);

res
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use itertools::{assert_equal, Itertools};
    use tracing::debug;
    use tracing::field::debug;
    use crate::day6::day6::{from_input_part_1, from_input_part_2};

    pub fn init_logger(){
            tracing_subscriber::fmt::init()
    }

    #[test]
    fn test_day_3_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");

        let res = from_input_part_1(input);


    }
    #[test]
    fn test_day_3_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");

        let res = from_input_part_2(input);

    }

}