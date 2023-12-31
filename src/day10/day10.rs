use std::borrow::Borrow;
use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::usize;


use itertools::{Itertools, unfold};


use tracing::{debug, info, instrument};


use crate::day10::day10::Pipes::{FilledGround, Ground, Horizontal, NorthEast, NorthWest, SouthEast, SouthWest, Start, Vertical};

#[derive(Debug,Ord, PartialOrd, Eq, PartialEq)]
#[derive(Clone)]
enum Pipes {
    Start,
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    Ground,
    FilledGround,
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

enum Corner {
    NW,
    NE,
    SW,
    SE
}
#[derive(Ord, PartialOrd, Eq, PartialEq,Copy, Clone,Debug)]

enum Direction {
    NORTH,
    SOUTH,
    EAST,
    WEST
}


impl Pipes{


    //(NORTH,SOUTH,EAST,WEST)
    pub fn connects_to(&self) -> (Option<Vec<Self>>,Option<Vec<Self>>,Option<Vec<Self>>,Option<Vec<Self>>) {
        match self {
            Pipes::Start => {(
                Some(vec![Vertical,SouthEast,SouthWest,Start]),
                Some(vec![Vertical,NorthEast,NorthWest,Start]),
                Some(vec![Horizontal,SouthWest,NorthWest,Start]),
                Some(vec![Horizontal,SouthEast,NorthEast,Start]),
            )}
            Pipes::Horizontal => {(
                None,
                None,
                Some(vec![Horizontal,SouthWest,NorthWest,Start]),
                Some(vec![Horizontal,SouthEast,NorthEast,Start]),
                )}
            Pipes::Vertical => {(
                Some(vec![Vertical,SouthEast,SouthWest,Start]),
                Some(vec![Vertical,NorthEast,NorthWest,Start]),
                None,
                None
                )}
            Pipes::NorthEast => {(
                Some(vec![Vertical,SouthEast,SouthWest,Start]),
                None,
                Some(vec![Horizontal,SouthWest,NorthWest,Start]),
                None,
                )}
            Pipes::NorthWest => {(
                Some(vec![Vertical,SouthEast,SouthWest,Start]),
                None,
                None,
                Some(vec![Horizontal,SouthEast,NorthEast,Start]),
                )
            }
            Pipes::SouthWest => {(
                None,
                Some(vec![Vertical,NorthEast,NorthWest,Start]),
                None,
                Some(vec![Horizontal,SouthEast,NorthEast,Start]),
                )}
            Pipes::SouthEast => {(
                None,
                Some(vec![Vertical,NorthEast,NorthWest,Start]),
                Some(vec![Horizontal,SouthWest,NorthWest,Start]),
                None,
                )}
            Pipes::Ground => {(None,None,None,None)}
            Pipes::FilledGround => {(None,None,None,None)}
        }
    }

}
#[derive(Clone,Debug,Eq, PartialEq)]
struct AbstractPosition<T> {
    pub value: T,
    x : i32,
    y : i32,
}

impl <T>AbstractPosition<T> {
    pub fn coordinate_into_tuple(&self) -> (i32, i32) {
        (self.x,self.y)
    }

    pub fn get_all_neighbours(&self) -> Vec<(Direction,(i32, i32))> {
        vec![
            (Direction::NORTH,(self.x,self.y-1)),
            (Direction::SOUTH,(self.x,self.y+1)),
            (Direction::WEST,(self.x-1,self.y)),
            (Direction::EAST,(self.x+1,self.y))
        ]
    }
}
#[derive(Debug,Clone)]
struct PipeMap(Vec<Vec<AbstractPosition<Pipes>>>);
trait DoubleVecMap<T> {
    fn get(&self, x:i32, y: i32) -> Option<&AbstractPosition<T>>;
    fn get_from_tuple(&self, pos : (i32 ,i32)) -> Option<&AbstractPosition<T>>;

    fn get_mut(&mut self, x:i32, y: i32) -> Option<&mut AbstractPosition<T>>;
    fn get_from_tuple_mut(&mut self, pos : (i32 ,i32)) -> Option<&mut AbstractPosition<T>>;

    fn set_pos(&mut self, pos : AbstractPosition<T>);
}

impl DoubleVecMap<Pipes> for PipeMap {
    fn get(&self, x:i32, y: i32) -> Option<&AbstractPosition<Pipes>> {
        if x <0 || y < 0 {return None}
        self.0.get(y as usize).and_then(|e|e.get(x as usize))
    }
    fn get_from_tuple(&self, pos : (i32 ,i32)) -> Option<&AbstractPosition<Pipes>> {
        self.get(pos.0,pos.1)
    }

    fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut AbstractPosition<Pipes>> {
        if x <0 || y < 0 {return None}
        self.0.get_mut(y as usize).and_then(|e|e.get_mut(x as usize))
    }

    fn get_from_tuple_mut(&mut self, pos: (i32, i32)) -> Option<&mut AbstractPosition<Pipes>> {
        self.get_mut(pos.0,pos.1)
    }

    fn set_pos(&mut self, pos: AbstractPosition<Pipes>) {
        let (x,y) = pos.coordinate_into_tuple();
        if pos.x <0 || pos.y < 0 {return}
        self.0[y as usize][x as usize] = pos
    }
}

impl PipeMap {

    pub fn find_starting_position(&self) -> Option<&AbstractPosition<Pipes>> {
        let x = self.0
            .iter().find_map(|x|{
            x.iter().find(|e| e.value == Pipes::Start)

        });
        x
    }

    pub fn calculate_all_points_and_scores(&self) -> HashMap<(i32, i32), i32> {
        let start = self.find_starting_position();
        let mut checked_nodes = HashMap::new();

        //
        if let Some(start)= start {

            //(counter,)
            let check_loop = unfold((0, vec![start]),|acc|{
                let mut new_to_check = Vec::new();
                acc.1.iter().for_each(|&e|{
                    if let std::collections::hash_map::Entry::Vacant(e) = checked_nodes.entry(e.coordinate_into_tuple()) {
                        e.insert(acc.0);
                    }

                    new_to_check.extend_from_slice(&self
                        .get_all_valid_connections_prom_point(e)
                        .iter().copied()
                        .filter(|&(d,e)| {
                            debug!("Before pre filter : {:?} , contains? = {} ",e, !checked_nodes.contains_key(&e.coordinate_into_tuple()));
                            !checked_nodes.contains_key(&e.coordinate_into_tuple())
                        })
                        .map(|(d,e)|e)
                        .collect_vec()
                    )
                });
                debug!("Score : {} ; old Checklist : {:?} ; newChecklist before DEDUP : {:?}" , acc.0 , acc.1 , new_to_check);
                new_to_check.dedup_by_key(|e|e.coordinate_into_tuple());
                debug!("Score : {} ; old Checklist : {:?} ; newChecklist after DEDUP : {:?}" , acc.0 , acc.1 , new_to_check);
                if(new_to_check.is_empty()){
                    None
                }else {
                    acc.0 += 1;
                    acc.1 = new_to_check;
                    Some(acc.clone())
                }
            });
            check_loop.last();
        }
        checked_nodes
    }

    pub fn get_all_valid_connections_prom_point(&self, position: &AbstractPosition<Pipes>) -> Vec<(Direction, &AbstractPosition<Pipes>)> {
        let connectionpoints = position.value.connects_to();
        let position_coordinate = position.coordinate_into_tuple();
        let mut valid_positions: Vec<(Direction,&AbstractPosition<Pipes>)> = Vec::new();
        debug!("Valid positions for {:?} are {:#?}" , position,connectionpoints);

        if let Some(north) = connectionpoints.0 {
            let pos_north = self.get_from_tuple((position_coordinate.0+0,position_coordinate.1-1));
            if let Some(pos) = pos_north {
                debug!("Connection North = {:?} ; Valid Types {:?}" , pos , north);
                if north.contains(&pos.value) {
                    valid_positions.push((Direction::NORTH,pos))
                }
            }
        };
        if let Some(south) = connectionpoints.1 {
            let pos_south = self.get_from_tuple((position_coordinate.0+0,position_coordinate.1+1));
            if let Some(pos) = pos_south{
                debug!("Connection South = {:?} ; Valid Types {:?}" , pos , south);
                if south.contains(&pos.value) {
                    valid_positions.push((Direction::SOUTH,pos))
                }
            }
        };
        if let Some(east) = connectionpoints.2 {
            let pos_east = self.get_from_tuple((position_coordinate.0+1,position_coordinate.1+0));
            if let Some(pos) = pos_east{
                debug!("Connection East = {:?} ; Valid Types {:?}" , pos , east);
                if east.contains(&pos.value) {
                    valid_positions.push((Direction::EAST,pos))
                }
            }
        };
        if let Some(west) = connectionpoints.3 {
            let pos_west = self.get_from_tuple((position_coordinate.0-1,position_coordinate.1+0));
            if let Some(pos) = pos_west{
                debug!("Connection West = {:?} ; Valid Types {:?}" , pos , west);
                if west.contains(&pos.value) {
                    valid_positions.push((Direction::WEST,pos))
                }
            }
        };
        debug!("Got those {:?} valid Positions from {:?} ",valid_positions,position);
        valid_positions
    }

    pub fn sub_map_of_pipe(&mut self) {
        let pipes_cords = self.calculate_all_points_and_scores()
            .iter()
            .map(|(&k,v)|k)
            .collect_vec();
        let minx = pipes_cords.iter().min_by_key(|v|v.0).map(|v|v.0).expect("Should have atleast 1 Point");
        let maxx = pipes_cords.iter().max_by_key(|v|v.0).map(|v|v.0).expect("Should have atleast 1 Point")-minx;
        let miny = pipes_cords.iter().min_by_key(|v|v.1).map(|v|v.1).expect("Should have atleast 1 Point");
        let maxy = pipes_cords.iter().max_by_key(|v|v.1).map(|v|v.1).expect("Should have atleast 1 Point") - miny;
        let mut all_needed_pipes = pipes_cords.iter().flat_map(|&c| { self.get_from_tuple(c) })
            .cloned().collect_vec();
        //Transform coordinates
        //all_needed_pipes.iter_mut().for_each(|e|{e.x -= minx ; e.y -=miny });
        //Clean Up to only Ground
        self.0.iter_mut().for_each(|e|{e.iter_mut().for_each(|e|{e.value = Ground})});
        //Replace Pipes again
        all_needed_pipes.iter().for_each(|e|self.set_pos(e.clone()));
        //Change Start Type to Horizontal / Corner / Vertical
        let mut starting_pos = self.find_starting_position().unwrap().clone();
        let conntecters =
            starting_pos.value.connects_to();
        let neighbours = self.get_all_valid_connections_prom_point(&starting_pos);
        let valid_connections = neighbours.iter()
            .filter(|(d,e)|
                match d {
                    Direction::NORTH => {
                        conntecters.clone().0.unwrap().contains(&e.value)
                    }
                    Direction::SOUTH => {
                        conntecters.clone().1.unwrap().contains(&e.value)
                    }
                    Direction::EAST => {
                        conntecters.clone().2.unwrap().contains(&e.value)
                    }
                    Direction::WEST => {
                        conntecters.clone().3.unwrap().contains(&e.value)
                    }

            })
            .map(|&(d,e)|d)
            .collect_vec();
        let new_type = if valid_connections.contains(&Direction::NORTH) && valid_connections.contains(&Direction::SOUTH) { Vertical }
        else if valid_connections.contains(&Direction::NORTH) && valid_connections.contains(&Direction::WEST) { NorthWest }
        else if valid_connections.contains(&Direction::NORTH) && valid_connections.contains(&Direction::EAST) { NorthEast }
        else if valid_connections.contains(&Direction::WEST) && valid_connections.contains(&Direction::SOUTH) { SouthWest }
        else if valid_connections.contains(&Direction::EAST) && valid_connections.contains(&Direction::SOUTH) { SouthEast }
        else {Horizontal};

        starting_pos.value = new_type;

        self.set_pos(starting_pos)






        // Now its usable for calculation
    }

    pub fn transform_all_to_filled(&mut self){

        #[derive(PartialEq)]
enum State {OUT, IN}

        impl State {
            fn toggle(&mut self) {
                *self = match self {
                    State::OUT => {State::IN}
                    State::IN => {State::OUT}
                };
            }
        }

        let mut state = State::OUT;
        self.0.iter_mut()
            .for_each(|e| {
                state = State::OUT;
                e.iter_mut().fold(0, |crossings, e| {
                    match e.value {
                        Start => {
                            //state.toggle()
                        }
                        Vertical => {
                            state.toggle()
                        }
                        Horizontal => {  }
                        NorthEast => {   }
                        NorthWest => {   }
                        SouthWest => { state.toggle() }
                        SouthEast => { state.toggle()  }
                        Ground => {
                            if state == State::OUT {
                                e.value = FilledGround
                            }

                        }
                        FilledGround => {
                            if state == State::IN {
                                //e.value = Ground;
                            }

                        }
                    }
                    crossings
                });
            })
    }
}


struct PointScore {
    x : usize,
    y : usize,
    score : u32
}

impl PartialEq for PointScore {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}






pub fn run_day_10_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);
    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_10_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input);
    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> i32 {
    info!("Running");
    let pipes = input
        .lines()
        .enumerate()
        .map(|(y,l)| {
             l
                 .chars()
                 .enumerate()
                 .map(|(x,c)| {
                     print!("{}",c);
                     AbstractPosition {
                        value: Pipes::from(c),
                        x : x as i32,
                        y : y as i32,
                     }
                 })
                 .collect_vec()
        })
        .collect_vec();
    let pipe_map = PipeMap(pipes);
    info!("Pre Processed Data:{:?}",pipe_map);
    let data = pipe_map.calculate_all_points_and_scores();
    info!("Resulting Data {:?}",data);

    let maximum_coordinate = data.iter().max_by_key(|e|e.1).unwrap();
    info!("Maximum Value = {}",maximum_coordinate.1);
    *maximum_coordinate.1
}


pub fn from_input_part_2(input : &str ) -> usize {

    info!("Running");
    let pipes = input
        .lines()
        .enumerate()
        .map(|(y,l)| {
            l
                .chars()
                .enumerate()
                .map(|(x,c)| {
                    print!("{}",c);
                    AbstractPosition {
                        value: Pipes::from(c),
                        x : x as i32,
                        y : y as i32,
                    }
                })
                .collect_vec()
        })
        .collect_vec();
    let mut pipe_map = PipeMap(pipes);
    //info!("Pre Processed Data:{:?}",pipe_map);
    //Generate subset of the Map
    println!("Before Map Slicing");
    pipe_map.0.iter().for_each(|e| {
        let e = e.iter().map(|e|match e.value {
            Ground => {'.'}
            FilledGround => {'I'}
            _ => {'X'}
        }).fold(String::new(),|acc, c|format!("{}{}",acc,c));
        println!("{:?}", e)
    });

    pipe_map.sub_map_of_pipe();
    //debug print of map
    println!("Draw of Map");
    pipe_map.0.iter().for_each(|e| {
        let e = e.iter().map(|e|match e.value {
            Ground => {'.'}
            FilledGround => {'I'}
            _ => {'X'}
        }).fold(String::new(),|acc, c|format!("{}{}",acc,c));
        println!("{:?}", e)
    });
    //transforming of fields
    pipe_map.transform_all_to_filled();
    println!("Draw of Map , After Transform");
    pipe_map.0.iter().for_each(|e| {
        let e = e.iter().map(|e|match e.value {

            Ground => {'I'}
            FilledGround => {'.'}
            Start => {'┼'}
            Vertical => {'│'}
            Horizontal => {'─'}
            NorthEast => {'└'}
            NorthWest => {'┘'}
            SouthWest => {'┐'}
            SouthEast => {'┌'}
        }).fold(String::new(), |acc, c|format!("{}{}", acc, c));
        println!("{:?}", e)
    });
    //pipe_map.0.iter().for_each(|e|println!("{:?}",e));
    //Count
    let fill_count = pipe_map.0.iter()
        .map(|e|{
            let x = e.iter()
                .filter(|e| { e.value == Ground })
                .count();
            x
        })
        .sum::<usize>();
    info!("Filled Amount Value = {:?}",fill_count);
    fill_count



}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    use tracing_test::traced_test;
    use super::{from_input_part_1, from_input_part_2};

    static INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(tracing_subscriber::fmt::init);
    }


    #[test]
    fn test_day_10_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let  input2 = include_str!("./testInput2.txt");
        let res = from_input_part_1(input);
        let res2 = from_input_part_1(input2);

        assert_eq!(4,res);
        assert_eq!(8,res2);



    }
    #[test]
    fn test_day_10_part_2(){
        init_logger();
        let  input = include_str!("./testInput3.txt");
        let  input2 = include_str!("./testInput4.txt");
        let  input3 = include_str!("./testInput5.txt");
        let res = from_input_part_2(input);
        let res2 = from_input_part_2(input2);
        let res3 = from_input_part_2(input3);

        assert_eq!(4,res);
        assert_eq!(8,res2);
        assert_eq!(10,res3);
    }

}