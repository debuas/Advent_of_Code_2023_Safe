use std::collections::{HashMap, HashSet};
use rayon::iter::ParallelIterator;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use glam::{IVec2, U64Vec2, Vec2, Vec2Swizzles};

use itertools::{Itertools};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{IntoParallelRefIterator};
use tracing::{debug, info, warn};
use tracing::field::debug;


pub fn run_day_15_part_1() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_15_part_2() {
    let  input = include_str!("input.txt");
    let res = from_input_part_2(input,1000000000);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}


trait AocHash {

    fn meta_hash(&self) -> u64;

}

impl AocHash for &str{
    fn meta_hash(&self) -> u64 {
        self
            .as_bytes()
            .iter()
            .fold(0,|acc,c|{
                (acc+ *c as u64)*17 % 256
            })
    }
}



pub fn from_input_part_1(input : &str) -> u64 {
    let res = input
        .split(',')
        .collect_vec()
        .into_par_iter()
        .map(|s| {
            debug!("String: '{}'",s );
            let res =s.meta_hash();
            debug!("hashed: '{}'",res );
            res
        }

        )
        .sum::<u64>();
    res
}

pub fn from_input_part_2(input : &str, amount : usize) -> usize {

    todo!()
}



#[cfg(test)]
mod tests {
    use tracing::info;
    use super::{AocHash, from_input_part_1, from_input_part_2};



    #[test_log::test]
    fn test_day_14_part_1(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(1320,res)



    }
    #[test_log::test]
    fn test_day_14_part_2(){
        let  input = include_str!("testInput1.txt");


        assert_eq!(64,64)
    }

    #[test_log::test]
    fn test_Hash_string(){

        let hash = "HASH".meta_hash();
        assert_eq!(hash,52)
    }

    #[test_log::test]
    fn test_day_14_part_2_set_cycles_test(){

    }


}