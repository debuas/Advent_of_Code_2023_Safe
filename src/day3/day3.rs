use std::cell::{RefCell, RefMut};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{RangeFrom, RangeInclusive};
use itertools::Itertools;
use tracing::{debug, info};
use tracing::field::debug;
use crate::day3::day3::SchemaDefinitions::{BLANK, DIGIT, GEAR, MARKER};

#[derive(Debug)]
struct SchematicSegment {
    line1 : String,
    line2 : String,
    line3 : String,
}
#[derive(Debug,Clone,Ord, PartialOrd, Eq, PartialEq,Hash)]
struct Position<T> {
    pub value: T,
    pub index_y : i32,
    pub index_beg : i32,
    pub index_end : i32,
}

//(x1,x2,y1,y2)
fn intersect<T,A>(rect1:&Position<T>,rect2: &Position<A>) -> bool {

    // Check if one rectangle is to the left of the other
    if rect1.index_end < rect2.index_beg-1 || rect2.index_end+1 < rect1.index_beg {
        return false;
    }

    // Check if one rectangle is above the other
    if rect1.index_y < rect2.index_y - 1 || rect2.index_y < rect1.index_y - 1 {
        return false;
    }
    // If the above conditions are not met, then the rectangles intersect
    true
}


impl SchematicSegment {

    pub fn from_slice(input : &[Option<&str>;3]) -> Self {
        let line1 = input[0].unwrap_or("").to_string()+".";
        let line2 = input[1].unwrap_or("").to_string()+".";
        let line3 = input[2].unwrap_or("").to_string()+".";
        Self{
            line1,
            line2,
            line3,
        }
    }

    pub fn read_part_numbers(&self, segment_index : i32) -> Vec<Position<u32>> {
        let mut number_pos: Vec<Position<u32>> = vec![];
        let mut symbol_pos: Vec<Position<bool>> = vec![];



        let mut mapfn =  |acc : i32 ,c: char,index_y, mut buffer : RefMut<String>| {
            if c.is_digit(10) {
                buffer.push(c);
            }else if c != '.' {
                symbol_pos.push(Position{
                    value: true,
                    index_y,
                    index_beg: acc+1,
                    index_end: acc+1,
                })
            }else {
                if let Some(x) = buffer.parse::<u32>().ok() {
                    number_pos.push(Position{
                        value: x,
                        index_y,
                        index_beg: acc+1-buffer.len() as i32,
                        index_end: acc,
                    });
                }
                buffer.clear()
            }
            ;acc+1
        };

        let  mut number_buf1 = RefCell::new(String::from(""));
        self.line1.chars()
            .fold(0,|acc,c| mapfn(acc,c,segment_index+1,number_buf1.borrow_mut()));
        let  mut number_buf2 = RefCell::new(String::from(""));
        self.line2.chars()
            .fold(0,|acc,c| mapfn(acc,c,segment_index+2,number_buf2.borrow_mut()));
        let  mut number_buf3 = RefCell::new(String::from(""));
        self.line3.chars()
            .fold(0,|acc,c| mapfn(acc,c,segment_index+3,number_buf3.borrow_mut()));

        //validate findall number in range yPos-1 -> yPos+1 , x-1 ,x+1

        println!("Positions numbs : {:?}",number_pos);
        println!("Positions chars : {:?}",symbol_pos);

        let part_numbers : Vec<Position<u32>>  = number_pos
            .iter()
            .filter(|e| {
                let any = &symbol_pos
                    .iter()
                    .any(|p| {
                        intersect(e,p)
                    });
                *any
            })
            .map(|e| e.clone())
            .collect();
        part_numbers
    }
}

struct Schematic {
    schema : Vec<(usize,Vec<(usize,SchemaDefinitions)>)>
}

#[derive(PartialEq)]
#[derive(Clone)]
enum SchemaDefinitions {
    BLANK,
    DIGIT(u32),
    MARKER,
    GEAR,
}
#[derive(Clone,Debug)]
struct Gear;

#[derive(Clone,Debug)]
struct AbstractPosition<T> {
    pub value: T,
    pub digit_position : Vec<(usize,usize)>
}
trait NumberPosition{
    fn append_number_with_position_from_digit(&self, digit : u32, index : (usize,usize) ) -> Self;
    fn new_from_number_and_index(digit : u32, index : (usize,usize)) -> Self;
}

impl NumberPosition for AbstractPosition<u32>{

    fn append_number_with_position_from_digit(&self, digit : u32, index : (usize,usize) ) -> Self{
        let mut new = self.clone();
        new.value = format!("{}{}", self.value, digit).parse::<u32>().unwrap();
        new.digit_position.push(index);
        new
    }
    fn new_from_number_and_index(digit : u32, index : (usize,usize)) -> Self {
        Self{ value: digit, digit_position: vec![index] }
    }
}

impl <T> AbstractPosition<T> {


    fn generate_scan_positions(&self) -> Vec<(usize, usize)> {
        let min_x = max(self.digit_position.iter().fold(None,|acc,e| {if let Some(x) = acc {Some(min(x,e.0))} else {Some(e.0)}}).unwrap() as isize-1,0) as usize;
        let max_x = self.digit_position.iter().fold(None,|acc,e| {if let Some(x) = acc {Some(max(x,e.0))} else {Some(e.0)}}).unwrap()+1;
        let y = self.digit_position.first().unwrap().1;
        let min_y = max(y as isize -1 ,0) as usize;
        let max_y = y+1;

        let mut pos = vec![];
        RangeInclusive::new(min_y,max_y).for_each(|y|{
            RangeInclusive::new(min_x,max_x).for_each(|x|{
                pos.push((x,y))
            })
        });
        pos
    }

    pub fn check_points_in_position(&self , to_check : &[(usize,usize)]) -> bool{

        to_check.iter()
            .fold(false,|acc,e|{
                acc || self.digit_position.contains(e)
            })
    }

}

impl Schematic {

    fn from_schematic_string(input : &str, definition_blank : &[char]) -> Self{
        let mut schema :Vec<(usize,Vec<(usize,SchemaDefinitions)>)> = Vec::new();
        input
            .lines()
            .enumerate()
            .for_each(|(index,line)|{
                let mut defs = Vec::new();
                (format!("{}{}",line,definition_blank[0])).chars().fold(0,|acc :usize,c|{
                    let def =
                    if let Some (digit) = c.to_digit(10){
                        DIGIT(digit)
                    }else if definition_blank.contains(&c) {
                        BLANK
                    }else if c == '*' {
                        GEAR
                    }
                    else {MARKER};
                    defs.push((acc,def));
                    acc+1
                });
                schema.push((index,defs))
            });
        Self { schema }
    }

    fn get_index(&self, x:usize, y:usize) -> Option<SchemaDefinitions> {
        let res = self.schema.get(y).and_then(|it| it.1.get(x).and_then(|it|Some(it.clone().1)));
        res
    }

    fn get_all_numbers(&self) -> Vec<AbstractPosition<u32>> {
        let mut collect = vec![];
        self.schema
            .iter()
            .for_each(|(y,v) | {
                let mut number : Option<AbstractPosition<u32>> = None;
                v.iter().for_each(|(x,v)|{
                    match v {
                        DIGIT(dig) => {if let Some(nu) = &number {
                                number = Some(nu.append_number_with_position_from_digit(*dig,(*x,*y)))
                        } else { number = Some(AbstractPosition::new_from_number_and_index(*dig, (*x, *y))) }}
                        _ => {
                            collect.push(number.clone());
                            number = None;
                        }
                    };
                });
            });
        let data : Vec<AbstractPosition<u32>> = collect.iter().flatten().map(|e|e.clone()).collect() ;
        data
    }

    fn get_all_gears(&self) -> Vec<AbstractPosition<Gear>> {
        let mut collect = vec![];
        self.schema
            .iter()
            .for_each(|(y,v) | {

                v.iter().for_each(|(x,v)|{
                    match v {
                        GEAR => collect.push(AbstractPosition{value: Gear,digit_position: vec![(*x,*y)] }),
                        _ => {}
                    };
                });
            });
        collect
    }

    fn validate_all_positions<'a , T>(&'a self, numbers : &'a[AbstractPosition<T>]) -> Vec<&AbstractPosition<T>>{
        numbers.iter()
            .filter(|&e|{
                let checkpos = e.generate_scan_positions();
                let res =checkpos.iter().find(|e| { if let Some(x) = self.get_index(e.0, e.1) {x == MARKER || x == GEAR} else { false }  });
                res.is_some()
            }).collect()

    }

    fn validate_all_gear_positions<'a>(&'a self, gears : &'a[AbstractPosition<Gear>], numbers : &[&AbstractPosition<u32>]) -> Vec<(&'a AbstractPosition<Gear>, Vec<u32>)> {
        gears.iter()
            .filter_map(|e|{
                let checkpos = e.generate_scan_positions();
                debug!("Position Check Gear : {:?}",checkpos);
                let res = numbers.iter().filter_map(|e| {if e.check_points_in_position(checkpos.as_slice()) {Some(e.value.clone())}else { None }}).collect_vec()  ;
                debug!("amount :{}",res.len() );
                if(res.len()==2) {
                    Some((e,res))
                } else { None }
            }).collect_vec()
    }

}



pub fn run_day_3_part_1(){
    let  input = include_str!("./input.txt");
    let res = from_input_numbers(input);
    println!("Result: {:?}", res)

}

pub fn run_day_3_part_2(){
    let  input = include_str!("./input.txt");
    let res = from_input_gears(input);
    println!("Result: {:?}", res)
}


fn from_input_numbers(input : &str) -> u32{
    let mut buffer : [Option<&str>;3] = [None,None,None];
    let mut schems: Vec<SchematicSegment> = vec![];

    let schma = Schematic::from_schematic_string(input,&['.']);

    let bind = schma.get_all_numbers();

    let res =schma.validate_all_positions(bind.as_slice());

    info!("Amount of entries : {}", res.len());

    res.iter().fold(0,|acc,e|acc+e.value)


}
fn from_input_gears(input : &str) -> u32{
    let mut buffer : [Option<&str>;3] = [None,None,None];
    let mut schems: Vec<SchematicSegment> = vec![];

    let schma = Schematic::from_schematic_string(input,&['.']);

    let numbers = schma.get_all_numbers();
    let gears = schma.get_all_gears();
    let valid_n = schma.validate_all_positions(numbers.as_slice()).clone();;
    let valid_g = schma.validate_all_gear_positions(gears.as_slice(),valid_n.as_slice());
    info!("Amount of numbers : {}", numbers.len());
    info!("Amount of valid numbers : {}", valid_n.len());
    info!("Amount of gears : {}", gears.len());
    info!("Amount of valid gears : {}", valid_g.len());

    valid_g.iter().fold(0,|acc,(pos,e)|acc+e.iter().fold(1,|acc,e|{debug!("l:{} r: {} pow : {}",acc, e, acc*e);acc*e}))


}



mod tests {
    use crate::day3::day3::{from_input_gears, from_input_numbers};

    #[test]
    fn test_part_1(){
        let  input = include_str!("./testinput1.txt");

        let res = from_input_numbers(input);

        assert_eq!(4361,res)
    }

    #[test]
    fn test_part_2(){
        tracing_subscriber::fmt::init();
        let  input = include_str!("./testinput1.txt");
        let res = from_input_gears(input);

        assert_eq!(467835,res)

    }

}