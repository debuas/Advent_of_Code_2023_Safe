use std::borrow::Borrow;
use std::any::Any;


use itertools::{Itertools, unfold};


use tracing::{debug, info, instrument};
use tracing::field::debug;



enum Pipes {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground
}

impl From<char> for Pipes{
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            _ => Self::Ground
        }
    }
}





pub fn run_day_10_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);
    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_11_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input);
    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str )   {


}


pub fn from_input_part_2(input : &str )   {

}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    use crate::day9::day9::{from_input_part_1, from_input_part_2};

    static INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(tracing_subscriber::fmt::init);
    }

    #[test]
    fn test_day_9_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(114,res);


    }
    #[test]
    fn test_day_9_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_2(input);

        assert_eq!(2,res);
    }

}