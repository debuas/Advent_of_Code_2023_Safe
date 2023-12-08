use std::any::Any;

use std::cmp::{Ordering};
use std::collections::HashMap;




use itertools::{Itertools};

use tracing::{debug, info};

use crate::day7::day7::Combination::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs};




pub fn run_day_8_part_1() {
    let  input = include_str!("./input.txt");
    let rounds = from_input_part_1(input);

    let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid);

    info!("Result :  {:?}" , res);
    println!("Result :  {:?}" , res)
}

pub fn run_day_8_part_2() {
    let  input = include_str!("./input.txt");
    let rounds = from_input_part_2(input) ;

    let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> Vec<""> {


    todo!("PART 1")
}
pub fn from_input_part_2(input : &str ) -> Vec<""> {

    debug!("{:?}",res);
    todo!("PART 2")
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    
    use crate::day7::day7::{Combination, from_input_part_1, from_input_part_2};

    const INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(||tracing_subscriber::fmt::init())
    }

    #[test]
    fn test_day_8_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");


    }
    #[test]
    fn test_day_8_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");



    }

}