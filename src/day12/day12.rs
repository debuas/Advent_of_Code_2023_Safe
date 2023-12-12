use rayon::iter::ParallelIterator;
use std::collections::{HashMap};

use itertools::{Itertools};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{IntoParallelRefIterator};
use tracing::{debug, info,};

use crate::day12::day12::Status::{Defect, Operational, Unknown};


pub fn run_day_12_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_12_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}
#[derive(Clone,Debug,PartialEq)]
enum Status {
    Operational,
    Defect,
    Unknown,
}
impl From<Status> for char {
    fn from(value: Status) -> Self {
        match value {
            Operational => { '.' }
            Defect => { '#' }
            Unknown => { '?' }
        }
    }
}
#[derive(Clone,Debug,PartialEq)]
enum CalculationResults<T,E> {
    EndOfLoop(T),
    ContinueLoop(E)
}


#[derive(Clone,Debug)]
struct Spring {
    data: Vec<Status>,
    defect_count: Vec<usize>
}

impl Spring {

    fn from_line(line : &str) -> Self {
        let split_line = line
            .split_whitespace()
            .collect_vec();
        let defect_count = split_line.last().unwrap().split(',').flat_map(|s|s.parse::<usize>()).collect_vec();

        let data = split_line
            .first()
            .unwrap()
            .chars()
            .flat_map(|c| match c {
                '?' => {Some(Unknown)},
                '.' => {Some(Operational)}
                '#' => {Some(Defect)}
                _ => {
                    unreachable!("Should not happen");
                    None
                }
            })
            .collect_vec()
            ;
        Self{defect_count,data}
    }

    fn from_line_p2(line : &str) -> Self {
        let split_line = line
            .split_whitespace()
            .collect_vec();
        let dc = split_line.last().unwrap();
        let n = format!("{},{},{},{},{}",dc,dc,dc,dc,dc);
        debug!("N : '{}' ", n);
        let defect_count = n.split(',').flat_map(|s|s.parse::<usize>()).collect_vec();

        let l = split_line.first().unwrap();
        let nl = format!("{}?{}?{}?{}?{}",l,l,l,l,l);
        debug!("NL : '{}' ", nl);
        let data = nl
            .chars()
            .flat_map(|c| match c {
                '?' => {Some(Unknown)},
                '.' => {Some(Operational)}
                '#' => {Some(Defect)}
                _ => {
                    unreachable!("Should not happen");
                    None
                }
            })
            .collect_vec()
            ;
        Self{defect_count,data}
    }

}


fn calculate(cache: &mut HashMap<(usize, usize, usize), usize>, s: &[Status], within: Option<usize>, remaining: &[usize]) -> usize{
    if s.is_empty() {
        return match (within, remaining.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == remaining[0] => 1,
            _ => 0
        };
    }
    if within.is_some() && remaining.is_empty() {
        return 0;
    }

    let key = (s.len(), within.unwrap_or(0), remaining.len());
    if let Some(&x) = cache.get(&key) {
        return x;
    }

    let out = match (&s[0], within) {
        (Operational, Some(x)) if x != remaining[0] => 0,
        (Operational, Some(_)) => calculate(cache, &s[1..], None, &remaining[1..]),
        (Operational, None)    => calculate(cache, &s[1..], None, remaining),
        (Defect, Some(_)) => calculate(cache, &s[1..], within.map(|x| x+1), remaining),
        (Defect, None)    => calculate(cache, &s[1..], Some(1), remaining),
        (Unknown, Some(x)) => {
            let mut ans = calculate(cache, &s[1..], within.map(|x| x+1), remaining);
            if x == remaining[0] {
                ans += calculate(cache, &s[1..], None, &remaining[1..])
            }
            ans
        }
        (Unknown, None) =>
            calculate(cache, &s[1..], Some(1), remaining) +
                calculate(cache, &s[1..], None, remaining),
    };
    cache.insert(key, out);
    out
}



pub fn from_input_part_1(input : &str) -> usize {
    let start_time = chrono::Utc::now();
    let springs = input
        .lines()
        .map(Spring::from_line)
        .collect_vec();
    let start_time_recursion = chrono::Utc::now();
    let res = springs
        .par_iter()
        .map(|s| {
            let mut cache = HashMap::new();
            let res = calculate(&mut cache, &s.data.clone(), None, s.defect_count.as_slice());
            cache.clear();
            debug!("Calculated {}",res);
            res
        })
        .sum::<usize>();
    let end_time = chrono::Utc::now();
    let time_spend = end_time-start_time;
    let time_recursion = end_time-start_time_recursion;
    info!("Total Time Spend    : {:?}", time_spend.to_std().unwrap());
    info!("Recursion Time Spend: {:?}", time_recursion.to_std().unwrap());
    res

}

pub fn from_input_part_2(input : &str) -> usize {

    let start_time = chrono::Utc::now();
    let springs = input
        .lines()
        .map(Spring::from_line_p2)
        .collect_vec();
    let start_time_recursion = chrono::Utc::now();
    let res = springs
        .par_iter()
        .map(|s| {
            let mut cache = HashMap::new();
            let res = calculate(&mut cache, &s.data.clone(), None, s.defect_count.as_slice());
            cache.clear();
            debug!("Calculated {}",res);
            res
        })
        .sum::<usize>();
    let end_time = chrono::Utc::now();
    let time_spend = end_time-start_time;
    let time_recursion = end_time-start_time_recursion;
    info!("Total Time Spend    : {:?}", time_spend.to_std().unwrap());
    info!("Recursion Time Spend: {:?}", time_recursion.to_std().unwrap());
    res

}


#[cfg(test)]
mod tests {

    use super::{from_input_part_1, from_input_part_2};



    #[test_log::test]
    fn test_day_12_part_1(){
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(21,res)



    }
    #[test_log::test]
    fn test_day_12_part_2(){
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_2(input);

        assert_eq!(525152,res)
    }

}