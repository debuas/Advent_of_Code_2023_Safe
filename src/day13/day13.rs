use rayon::iter::ParallelIterator;
use std::collections::{HashMap};

use itertools::{Itertools};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{IntoParallelRefIterator};
use tracing::{debug, info,};


pub fn run_day_13_part_1() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_13_part_2() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}


enum Ground{
    Rock,
    Ash
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        todo!()
    }
}





pub fn from_input_part_1(input : &str) -> usize {



    todo!("Calculate")
}


#[cfg(test)]
mod tests {

    use super::{from_input_part_1,};



    #[test_log::test]
    fn test_day_13_part_1(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(21,res)



    }
    #[test_log::test]
    fn test_day_13_part_2(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(525152,res)
    }

}