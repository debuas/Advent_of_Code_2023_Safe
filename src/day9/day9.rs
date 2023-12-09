use std::borrow::Borrow;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use itertools::{Itertools, unfold};


use tracing::{debug, info, warn};
use tracing::field::debug;


pub fn run_day_8_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);
    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}

pub fn run_day_8_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input);
    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str )  {

}


pub fn from_input_part_2(input : &str )  {

}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    use crate::day8::day8::{from_input_part_1, from_input_part_2};

    static INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(tracing_subscriber::fmt::init);
    }

    #[test]
    fn test_day_8_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);



    }
    #[test]
    fn test_day_8_part_2(){
        init_logger();
        let  input = include_str!("./testInput2.txt");

    }

}