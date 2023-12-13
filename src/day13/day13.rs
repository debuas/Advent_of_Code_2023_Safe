use std::cmp::max_by;
use rayon::iter::ParallelIterator;
use std::collections::{HashMap};
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use chrono::expect;
use grid::{grid, Grid};

use itertools::{Itertools};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{IntoParallelRefIterator};
use tracing::{debug, info, warn};
use tracing::field::debug;
use crate::day13::day13::Ground::{Ash, Rock};


pub fn run_day_13_part_1() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input,0);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_13_part_2() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input,1);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}


#[derive(Clone,Copy,Default,PartialEq)]

#[derive(Debug,Hash)]
enum Ground{
    Rock = 1,
    #[default]
    Ash = 0
}

impl From<char> for Ground {
    fn from(value: char) -> Self {
        match value {
            '.' => Ash,
            '#' => Rock,
            _ => Ash,
        }

    }
}


struct SolarArray(Grid<Ground>);

impl SolarArray{

    fn from_grounds_vec(input : &[Vec<Ground>]) -> Self{
        debug!("{:#?}",input);
        let mut grid: Grid<Ground> = Grid::<Ground>::new(0,0);

        input.iter()
            .enumerate()
            .for_each(|(n,r)|{
                grid.insert_row(n, r.to_owned())
            });
        debug!("{:?}",grid);
        Self(grid)
    }
    pub fn calculate_numeric_and_solve(&self, offset : u32) -> Option<usize> {
        let cols = self.0.iter_cols()
            .map(|e| {
                let mut c = 0;
                e.for_each(|v|{
                    c = (c << 1) | ((v == &Rock) as u32)
                });
                c
            })
            .collect_vec();
        let rows = self.0.iter_rows()
            .map(|e| {
                let mut r = 0;
                e.for_each(|v|{
                    r = (r << 1) | ((v == &Rock) as u32)
                });
                r
            })
            .collect_vec();
        self.scan_row_col(&rows,&cols, offset)
    }

    fn scan_row_col(&self,rows : &[u32], cols : &[u32] ,offset : u32 ) -> Option<usize>{
        if let Some(x) = check(&cols,offset) {return Some(x) }
        if let Some(x) = check(&rows,offset) {return Some(100 * x) }
        None
    }

}

fn check(v : &[u32], offset : u32) -> Option<usize> {
    for value in 1..v.len() {
        if check_mirror(v,value,offset){
            return Some(value)
        }
    };
    None
}



fn check_mirror(rc : &[u32], i : usize ,offset : u32) -> bool {
    (0..i)
        .rev()
        .zip(i..rc.len())
        .map(|(a,b)| {
            (rc[a] ^ rc[b]).count_ones()
        })
        .sum::<u32>() == offset

}

pub fn from_input_part_1(input : &str,offset : u32) -> usize {
    let ground = input
        .lines()
        .map(|l|{
            l
                .chars()
                .map(Ground::from)
                .collect_vec()
        })
        .collect_vec();
    let ground = ground
        .split(|l|l.is_empty())
        .map(SolarArray::from_grounds_vec)
        .collect_vec();

    let scores = ground.iter()
        .map(|grid| grid.calculate_numeric_and_solve(offset).unwrap())
        .sum();


    debug!("{:#?}",scores);

    scores
}


#[cfg(test)]
mod tests {

    use super::{from_input_part_1,};



    #[test_log::test]
    fn test_day_13_part_1(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input,0);

        assert_eq!(405,res)



    }
    #[test_log::test]
    fn test_day_13_part_2(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input,1);

        assert_eq!(400,res)
    }

}