use std::collections::{HashMap, HashSet};
use glam::{I64Vec2};
use itertools::{Itertools};
use tracing::{debug, info,};



pub fn run_day_12_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_12_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}


pub fn from_input_part_1(input : &str)  {

}
#[cfg(test)]
mod tests {
    use std::sync::Once;
    use itertools::Itertools;
    use tracing_test::traced_test;
    use super::{from_input_part_1};

    static INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(tracing_subscriber::fmt::init);
    }

#[traced_test]
    #[test]
    fn test_day_12_part_1(){
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);




    }
    #[traced_test]
    #[test]
    fn test_day_12_part_2(){
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);

    }

}