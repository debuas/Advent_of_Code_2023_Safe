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
use crate::day15::day15::BoxM::{Insert, Remove};


pub fn run_day_15_part_1() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_15_part_2() {
    let  input = include_str!("input.txt");
    let res = from_input_part_2(input);

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
#[derive(Clone,Debug)]
enum BoxM{
    Remove(String),
    Insert(BoxV)
}
#[derive(Clone,Default,Debug)]
struct BoxV(String, u32);

impl BoxM{

    fn from_string(input: &str) -> BoxM{
       if input.contains('-') {
           let res = input.replace("-","");
           Remove(res)
       }else {
           let res = input.split('=').collect_vec();
           Insert(BoxV(res.first().unwrap().to_string(), res.last().unwrap().to_string().parse::<u32>().unwrap()))
       }
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

pub fn from_input_part_2(input : &str) -> usize {
    let commands = input
        .split(',')
        .collect_vec()
        .into_iter()
        .map(|s| {
            debug!("String: '{}'",s );
            let b = BoxM::from_string(s);
            let res = match &b {
                Remove(a) => {(a.as_str().meta_hash(),b)}
                Insert(v) => {(v.0.as_str().meta_hash(),b)}
            };
            debug!("hashed: '{:?}'",res );
            res
        })
        .collect_vec()
        ;
    let mut boxes: HashMap<u64,Vec<BoxV>> = HashMap::new();
    commands.iter()
        .for_each(|(c,b)|{
            match b {
                Remove(b) => {
                    if let Some(v) =boxes.get_mut(c){
                        if let Some((p,_)) = v.iter().find_position(|i| i.0 == b.as_str()) {
                            v.remove(p);
                        }
                    }
                }
                Insert(b) => {
                    if let Some(v) = boxes.get_mut(c){
                        if let Some((p,_)) = v.iter().find_position(|i| i.0 == b.0) {
                            v.get_mut(p).unwrap().1 = b.1
                        } else {
                            v.push(b.clone());
                        }
                    }else {
                        boxes.insert(*c,vec![b.clone()]);

                    }
                }
            }
        });
    debug!("Boxes : {:#?}",boxes);

    let res = boxes.iter()
        .enumerate()
        .fold(0,|acc,(i,(k,v))|{
            let res = acc + v.iter().enumerate().fold(0,|acc,(i2,b)|{
                let res =
                    acc +((k+1) as usize * (i2+1)*b.1 as usize) as usize;
                debug!("Box :  {} index {} = {} , v = {} , key : {}",k,i2,b.1,res,b.0  );
                res
            });
            debug!("Res = {}",res);
            res

        });

    res
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
        let res = from_input_part_2(input, /* usize */);

        assert_eq!(145,res)
    }

    #[test_log::test]
    fn test_Hash_string(){

        let hash = "HASH".meta_hash();
        let hash2 = "rn".meta_hash();
        assert_eq!(hash,52);
        assert_eq!(hash2,0)
    }




}