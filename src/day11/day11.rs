use std::collections::{HashMap, HashSet};
use glam::{I64Vec2};
use itertools::{Itertools};
use tracing::{debug, info,};
use crate::day11::day11::Observation::{Galaxy, Space};


pub fn run_day_11_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input,2);
    let sum = res.iter().map(|e|{*e.1}).collect_vec().iter().sum::<u64>();
    info!("Result Part 1 :  {:?}" , sum);
    println!("Result Part 1:  {:?}" , sum)
}

pub fn run_day_11_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input,1000000);
    let sum = res.iter().map(|e|{*e.1}).collect_vec().iter().sum::<u64>();
    info!("Result Part 2 :  {:?}" , sum);
    println!("Result Part 2:  {:?}" , sum)
}

enum Observation {
    Galaxy(I64Vec2),
    Space(I64Vec2)

}

pub fn from_input_part_1(input : &str, factor : i64) -> HashMap<(I64Vec2, I64Vec2), u64> {
    // All Galaxies
    let galaxy_vec = input
        .lines()
        .enumerate()
        .flat_map(|(y, l)| {
            l
                .chars()
                .enumerate()
                .map(|(x, c)|
                    match c {
                        '#' => {
                            Galaxy(I64Vec2 { x: x as i64, y: y as i64 })
                        }
                        '.' => {
                            Space(I64Vec2 { x: x as i64, y: y as i64 })
                        }
                        _ => { Space(I64Vec2::default()) }
                    }
                ).collect_vec()
        })
        .collect_vec();

    let void_x: HashSet<i64> = HashSet::from_iter(galaxy_vec.iter().filter_map(|e| { if let Space(v) = e { Some(v.x) } else { None } }));
    let void_y: HashSet<i64> = HashSet::from_iter(galaxy_vec.iter().filter_map(|e| { if let Space(v) = e { Some(v.y) } else { None } }));
    let galaxy_x: HashSet<i64> = HashSet::from_iter(galaxy_vec.iter().filter_map(|e| { if let Galaxy(v) = e { Some(v.x) } else { None } }));
    let galaxy_y: HashSet<i64> = HashSet::from_iter(galaxy_vec.iter().filter_map(|e| { if let Galaxy(v) = e { Some(v.y) } else { None } }));
    //get all intersections beetween Galaxies for expansion x
    let expansion_voids_x = void_x.difference(&galaxy_x).collect_vec();
    let expansion_voids_y = void_y.difference(&galaxy_y).collect_vec();

    info!("{} expansions in X count {}x",&expansion_voids_x.clone().len(),factor);
    info!("{:?} expansions in X count {}x",expansion_voids_x,factor);
    info!("{} expansions in y count {}x",expansion_voids_y.clone().len(),factor);
    info!("{:?} expansions in y count {}x",expansion_voids_y,factor);
    //Expand all galaxies
    let mut galaxys = galaxy_vec.iter().flat_map(|e| { if let Galaxy(v) = e { Some(v) } else { None } }).copied().collect_vec();
    //Transformed Galaxie for empty space
    debug!("Galaxies Before : {:?}",galaxys);
    galaxys.iter_mut()
        .for_each(|o|{
            let x_amount = &expansion_voids_x.iter().fold(0i64,|acc,e|if e <= &&o.x {acc +1} else { acc });
            let y_amount = &expansion_voids_y.iter().fold(0i64,|acc,e|if e <= &&o.y {acc +1} else { acc });
            o.x += if x_amount == &0 {0} else {(x_amount * (factor-1))} ;
            o.y += if y_amount == &0 {0} else {(y_amount * (factor-1))} ;
        });
    debug!("Galaxies After : {:?}",galaxys);
    let distances  = galaxys.iter()
        .combinations(2)
        .map(|a| {
            let first = *a.first().unwrap().clone();
            let last = *a.last().unwrap().clone();
            let x =first.x.abs_diff(last.x);
            let y =first.y.abs_diff(last.y);
            ((first, last),
                 x+y
            )
        })
        .collect_vec()
        ;
    let distance_map : HashMap<(I64Vec2, I64Vec2), u64> = HashMap::from_iter(distances);
    debug!("Distances : {:#?}",distance_map);
    distance_map
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
    fn test_day_11_part_1(){
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input,2);
        let sum = res.iter().map(|e|{*e.1}).collect_vec().iter().sum::<u64>();
        //info!("Sum {:?}",sum)
        assert_eq!(sum,374)



    }
    #[traced_test]
    #[test]
    fn test_day_11_part_2(){
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input,10);
        let res2 = from_input_part_1(input,100);
        let sum = res.iter().map(|e|{*e.1}).collect_vec().iter().sum::<u64>();
        let sum2 = res2.iter().map(|e|{*e.1}).collect_vec().iter().sum::<u64>();
        //info!("Sum {:?}",sum)
        assert_eq!(sum,1030);
        assert_eq!(sum2,8410)
    }

}