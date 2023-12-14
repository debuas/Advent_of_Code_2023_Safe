use std::collections::{HashMap, HashSet};
use rayon::iter::ParallelIterator;

use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use glam::{IVec2, U64Vec2, Vec2, Vec2Swizzles};

use grid::{Grid};

use itertools::{Itertools};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::{IntoParallelRefIterator};
use tracing::{debug, info, warn};
use tracing::field::debug;
use crate::day14::day14::Direction::{East, North, South, West};
use crate::day14::day14::Rocks::{Cube, Round};


pub fn run_day_14_part_1() {
    let  input = include_str!("input.txt");
    let res = from_input_part_1(input);

    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_14_part_2() {
    let  input = include_str!("input.txt");
    let res = from_input_part_2(input,1000000000);

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
            Direction::South => {self.tilt_south()}
            Direction::East => {self.tilt_east()}
            Direction::West => {self.tilt_west()}
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

    fn tilt_south(&mut self){
        debug!("Rocksize before Tilt : {}" ,self.round_rocks.len());
        for i in (0..self.size_y).rev(){
            //get all cubes and rounds in range
            let collideables = self.cube_rocks.iter().chain(self.round_rocks.iter()).filter(|e| { e.y > i as i32 }).collect_vec();
            //get all rounds of line
            let rocks_of_axis = self.round_rocks.iter().cloned().filter(|e|{e.y == i as i32}).collect_vec();
            let new_rock_pos = rocks_of_axis.iter().map(|r|{
                //get all collideable rocks with same x
                let cols = collideables.iter().filter(|e|e.x == r.x).collect_vec();
                if cols.len() == 0 {
                    IVec2::new(r.x, (self.size_y - 1) as i32)
                }else {
                    //get to position with smallest distance
                    let min = cols.iter().min_by_key(|e|r.y.abs_diff(e.y)).expect("There should atleast be one").y;
                    IVec2::new(r.x,min-1)
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

    fn tilt_east(&mut self){
        debug!("Rocksize before Tilt : {}" ,self.round_rocks.len());
        for i in (0..self.size_x).rev(){
            //get all cubes and rounds in range
            let collideables = self.cube_rocks.iter().chain(self.round_rocks.iter()).filter(|e| { e.x > i as i32 }).collect_vec();
            //get all rounds of line
            let rocks_of_axis = self.round_rocks.iter().cloned().filter(|e|{e.x == i as i32}).collect_vec();
            let new_rock_pos = rocks_of_axis.iter().map(|r|{
                //get all collideable rocks with same x
                let cols = collideables.iter().filter(|e|e.y == r.y).collect_vec();
                if cols.len() == 0 {
                    IVec2::new( (self.size_x - 1) as i32,r.y)
                }else {
                    //get to position with smallest distance
                    let min = cols.iter().min_by_key(|e|r.x.abs_diff(e.x)).expect("There should atleast be one").x;
                    IVec2::new(min-1,r.y)
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

    fn tilt_west(&mut self){
        debug!("Rocksize before Tilt : {}" ,self.round_rocks.len());

        for i in 0..self.size_x{
            //get all cubes and rounds in range
            let collideables = self.cube_rocks.iter().chain(self.round_rocks.iter()).filter(|e| { e.x < i as i32 }).collect_vec();
            //get all rounds of line
            let rocks_of_axis = self.round_rocks.iter().cloned().filter(|e|{e.x == i as i32}).collect_vec();
            let new_rock_pos = rocks_of_axis.iter().map(|r|{
                //get all collideable rocks with same x
                let cols = collideables.iter().filter(|e|e.y == r.y).collect_vec();
                if cols.len() == 0 {
                    IVec2::new(0,r.y)
                }else {
                    //get to position with smallest distance
                    let min = cols.iter().min_by_key(|e|r.x.abs_diff(e.x)).expect("There should atleast be one").x;
                    IVec2::new(min+1,r.y)
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

    pub fn cycle(&mut self){
        self.tilt_by_direction(North);
        self.tilt_by_direction(West);
        self.tilt_by_direction(South);
        self.tilt_by_direction(East);
    }

    pub fn draw(&self){
        let mut lines = vec![];
        for y in 0..self.size_y{
            let mut st = "".to_string();
            for x in 0..self.size_x {
                if let Some(cube) = self.cube_rocks.iter().find(|e|e.x==x as i32 && e.y == y as i32){
                    st.push('#')
                } else if let Some(cube) = self.round_rocks.iter().find(|e|e.x==x as i32 && e.y == y as i32){
                    st.push('O')
                } else { st.push('.') } ;
            }
            lines.push(st)
        }
        lines.iter()
            .for_each(|l|info!("{}",l))
    }

    pub fn cycle_by_amount(&mut self, amount : usize){
        let mut seen_first = HashMap::new();
        let backup_start = self.round_rocks.clone();
        let mut id_counter = HashMap::new();
        //let mut cycle_appearance = HashMap::new();
        let mut first_duplicate = None;

        let mut loopIndex = vec![];

        'this: for  i in (0..amount){
            //check if set contains:
            debug!("Iteration : {}", i);
            let id =self.round_rocks.iter().copied().map(|e|e.as_u64vec2()).sum::<U64Vec2>();
            match id_counter.entry(id) {
                std::collections::hash_map::Entry::Occupied(mut e) => {

                    warn!("LOOP on Index : {} last on {:?}",i,loopIndex.last());
                    loopIndex.push(i);
                    e.insert(2usize);
                    first_duplicate = Some((i,id));
                    break 'this;

                }
                std::collections::hash_map::Entry::Vacant(e) => {

                    seen_first.insert(id,(i,self.round_rocks.clone()));
                    e.insert(1usize);
                }
            }
            self.cycle()

        }

        if let Some((index,key)) = first_duplicate{
            // Loop Start , loop index(index) => loop start - index => Amount of a cycle
            // iterations - loop start => simulation size
            // sim size % cycle amount => realCycles from loop
            // loop start + real Cyclces
            let loop_start = seen_first[&key].0;
            let cycle_len = index.abs_diff(loop_start);
            let sim_size = amount - loop_start;
            let real_cycle = sim_size % cycle_len;
            let iter_run = loop_start+real_cycle;

            //Get the index from the cached ones
            self.round_rocks = seen_first.iter().find(|(k,(i,v))|i==&(iter_run)).map(|(k,(i,v))|v).cloned().unwrap();
        }


        println!("Seen combinations : {}" , id_counter.len());

    }

}


pub fn from_input_part_1(input : &str) -> usize {

    let mut arr = SupportArray::from_lines(input);

    arr.tilt_by_direction(Direction::North);
    let res = arr.calculate_stress_by_direction(Direction::North) as usize;
    arr.draw();
    res
}

pub fn from_input_part_2(input : &str, amount : usize) -> usize {
    let mut arr = SupportArray::from_lines(input);

    arr.cycle_by_amount(amount);
    let res = arr.calculate_stress_by_direction(Direction::North) as usize;
    arr.draw();

    res
}

pub fn from_input_calculate_score(input : &str) -> usize {
    let mut arr = SupportArray::from_lines(input);

    let res = arr.calculate_stress_by_direction(Direction::North) as usize;
    arr.draw();

    res
}

#[cfg(test)]
mod tests {
    use tracing::info;
    use super::{from_input_part_1, from_input_part_2,from_input_calculate_score};



    #[test_log::test]
    fn test_day_14_part_1(){
        let  input = include_str!("testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(136,res)



    }
    #[test_log::test]
    fn test_day_14_part_2(){
        let  input = include_str!("testInput1.txt");
        println!("Running Test with : {} ",1000000000);
        let res = from_input_part_2(input,1000000000);

        assert_eq!(64,res)
    }
    #[test_log::test]
    fn test_day_14_part_2_set_cycles_test(){
        let  input_sample1 = include_str!("testInput2.txt");
        let  input_sample2 = include_str!("testInput3.txt");
        let  input_sample3 = include_str!("testInput4.txt");
        let  input = include_str!("testInput1.txt");
        info!("Running Sample");
        let sample1 = from_input_calculate_score(input_sample1);
        let sample2 = from_input_calculate_score(input_sample2);
        let sample3 = from_input_calculate_score(input_sample3);
        info!("Running Test");
        let res1 = from_input_part_2(input,1);
        let res2 = from_input_part_2(input,2);
        let res3 = from_input_part_2(input,3);

        assert_eq!(sample1,res1);
        assert_eq!(sample2,res2);
        assert_eq!(sample3,res3)
    }


}