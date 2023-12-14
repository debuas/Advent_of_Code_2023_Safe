use std::collections::HashSet;
use rayon::iter::ParallelIterator;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use glam::{IVec2, Vec2, Vec2Swizzles};

use grid::{Grid};

use itertools::{Itertools};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{IntoParallelRefIterator};
use tracing::{debug, info,};
use tracing::field::debug;
use crate::day14::day14::Rocks::{Cube, Round};


pub fn run_day_14_part_1() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_14_part_2() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}


enum Rocks {
    Round(IVec2),
    Cube(IVec2)
}

enum Direction {
    North,
    South,
    East,
    West
}

struct SupportArray{
    size_x : usize,
    size_y : usize,
    cube_rocks : HashSet<IVec2>,
    round_rocks : HashSet<IVec2>
}

impl SupportArray {

    pub fn from_lines(input : &str) -> Self {
        let crr = input
            .lines()
            .enumerate()
            .flat_map(|(y,e)|{
                e
                    .chars()
                    .enumerate()
                    .flat_map(|(x,c)|{
                        match c {
                            '#' => Some(Cube(IVec2::new(x as i32,y as i32))),
                            'O' => Some(Round(IVec2::new(x as i32,y as i32))),
                            _ => None,
                        }
                    }).collect_vec()
            }).collect_vec();
        let size_x = input.lines().collect_vec().first().unwrap().len();
        let size_y = input.lines().collect_vec().len();
        let cube_rocks = crr.iter().filter_map(|e| if let Cube(e) = e {Some(e.to_owned())} else { None }).collect();
        let round_rocks = crr.iter().filter_map(|e| if let Round(e) = e {Some(e.to_owned())} else { None }).collect()
        ;
        Self{
            size_x,
            size_y,
            cube_rocks,
            round_rocks,
        }
    }

    pub fn tilt_by_direction(&mut self,direction: Direction){
        match direction {
            Direction::North => {
                self.tilt_north()
            }
            Direction::South => {todo!("NIPJ")}
            Direction::East => {todo!("NIPJ")}
            Direction::West => {todo!("NIPJ")}
        }
    }

    pub fn calculate_stress_by_direction(&self,direction: Direction) -> u32{
        match direction {
            Direction::North => {
                self.round_rocks.iter().map(|r|(r.y).abs_diff(self.size_y as i32)).sum()
            }
            Direction::South => {todo!("NIPJ")}
            Direction::East => {todo!("NIPJ")}
            Direction::West => {todo!("NIPJ")}
        }
    }

    fn tilt_north(&mut self){
        debug!("Rocksize before Tilt : {}" ,self.round_rocks.len());
        for i in 0..self.size_y{
            //get all cubes and rounds in range
            let collideables = self.cube_rocks.iter().chain(self.round_rocks.iter()).filter(|e| { e.y < i as i32 }).collect_vec();
            //get all rounds of line
            let rocks_of_axis = self.round_rocks.iter().cloned().filter(|e|{e.y == i as i32}).collect_vec();
            let new_rock_pos = rocks_of_axis.iter().map(|r|{
                //get all collideable rocks with same x
                let cols = collideables.iter().filter(|e|e.x == r.x).collect_vec();
                if cols.len() == 0 {
                     IVec2::new(r.x,0)
                }else {
                    //get to position with smallest distance
                    let min = cols.iter().min_by_key(|e|r.y.abs_diff(e.y)).expect("There should atleast be one").y;
                    IVec2::new(r.x,min+1)
                }
            }).collect_vec();
            // Remove Old Rocks
            for e  in  rocks_of_axis{
                &self.round_rocks.remove(&e);
            };
            // Add New Rocks
            new_rock_pos.iter().for_each(|e|{
                &self.round_rocks.insert(*e);
            })
        }
        debug!("Rocksize after  Tilt : {}" ,self.round_rocks.len());
    }

}

fn vec_collide_axis(a : IVec2 , b : IVec2 , destination : IVec2){


}






pub fn from_input_part_1(input : &str) -> usize {

    let mut arr = SupportArray::from_lines(input);

    arr.tilt_by_direction(Direction::North);
    let res = arr.calculate_stress_by_direction(Direction::North) as usize;

    res
}


#[cfg(test)]
mod tests {

    use super::{from_input_part_1,};



    #[test_log::test]
    fn test_day_14_part_1(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(136,res)



    }
    #[test_log::test]
    fn test_day_14_part_2(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(400,res)
    }

}