use std::cmp::max;
use std::ops::Add;

use serde::{Deserialize, Serialize};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};
use tracing::info;


#[derive(Serialize_enum_str,Deserialize_enum_str,Debug,Clone,)]
#[serde(rename_all ="lowercase")]
enum DrawColor {
    Red,
    Green,
    Blue,
}
#[derive(Serialize,Deserialize,Debug,Clone)]
struct Record{
    amount : u32,
    color : DrawColor
}
#[derive(Serialize,Deserialize,Debug,Clone)]
pub struct Game{
    id : u32
}
#[derive(Serialize,Deserialize,Debug,Clone,Default,)]
pub struct GameStat{
    pub red:u32,
    pub green:u32,
    pub blue:u32,
}

impl GameStat{

    fn from_record(record: &Record) -> Self{
        let mut stat = GameStat::default();
        match record.color {
            DrawColor::Red => {stat.red=record.amount}
            DrawColor::Green => {stat.green=record.amount}
            DrawColor::Blue => {stat.blue=record.amount}
        }
        stat
    }
    fn from_records(records :&[Record])->Self{
        records
            .iter()
            .fold(GameStat::default(), |acc,r|{
                acc.add(GameStat::from_record(r))
            })
    }

    pub fn compare_condition(condition : &Self, stats : &Self) -> bool{
        stats.red <= condition.red
            && stats.green <= condition.green
            && stats.blue <= condition.blue

    }

    pub fn sum(&self) -> u32 {
        self.red+self.green+self.blue
    }

    pub fn select_max(self,rhs : &Self) ->  Self{
        Self {
            red: max(self.red,rhs.red),
            green: max(self.green,rhs.green),
            blue: max(self.blue,rhs.blue),
        }
    }

    pub fn power(&self) -> u32 {
        self.blue*self.green*self.red
    }
}

impl Add for GameStat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            red: self.red+rhs.red,
            green: self.green+rhs.green,
            blue: self.blue+rhs.blue,
        }
    }
}


pub fn run_day_2_part1(){
    info!("Running day 1 , Part 1");
    let input = include_str!("./input1.txt");
    info!("Loaded Input File, lines : {}", input.lines().count());
    info!("Loaded Input File, Total String size : {}", input.len());
    let condition = GameStat{
        red: 12,
        green: 13,
        blue: 14,
    };
    info!("Condition ; {:?} ; Sum : {}",&condition , &condition.sum());
    let stats = solve_from_input(input);


    let res = sum_of_ids_by_condition(&condition,&stats);

    info!("Result = '{}'", res);
    println!("Result = '{}'", res);
}

pub(crate) fn solve_from_input(input : &str) -> Vec<(Game, GameStat)> {
    let game_stats : Vec<(Game,GameStat)> = input
        .lines()
        .map(|l| {
            let (game,entries) = split_game_from_line(l);
            let stat = entries
                .iter()
                .map(|&e| count_items_per_draw(e))
                .fold(GameStat::default(),|acc,e| {acc.select_max(&e) });
            info!("{:?} ; {:?} ; Sum : {}",game,&stat , &stat.sum());
            (game,stat)
        })
        .collect();
    game_stats
}

fn split_game_from_line(input : &str) -> (Game,Vec<&str>){
    let tmp : Vec<&str> = input.split(':').collect();
    info!("{:?}",tmp);
    let game = Game { id: tmp.first().unwrap().replace("Game ", "").parse::<u32>().unwrap() };
    println!("{:?}",game);

    (game,tmp.last().unwrap().split(';').collect())
}


fn count_items_per_draw(input : &str) -> GameStat {
    let res :Vec<&str> = input.split(',').collect();
    //map to pair (u32,Color)
    let records : Vec<Record> = res.
        iter()
        .map(|&i| {
            let tmp : Vec<&str> = i.trim().split(' ').collect();
            let amount = tmp.first().unwrap().parse::<u32>().unwrap();
            let color = tmp.last().unwrap().parse::<DrawColor>().unwrap();
            Record  {amount,color}
        })
        .collect();
    //Sum Up everything to per Game Color
    
    GameStat::from_records(&records)
}

fn sum_of_ids_by_condition(condition: &GameStat, game_stats : &[(Game,GameStat)]) -> u32 {
    game_stats
        .iter()
        .fold(0 , |acc,(game,stat)|{
            if GameStat::compare_condition(condition,stat) {acc+ game.id} else { acc }
        })
}
#[cfg(test)]
mod tests {

    use super::*;
    

    #[test]
    fn test(){
        let input = include_str!("./testInput.txt");
        let mut x =solve_from_input(input);
        let condition = GameStat {
            red: 12,
            green: 13,
            blue: 14,
        };
        let mut checklist = vec![true,true,false,false,true];
        println!("{:?} ",x);
        assert_eq!(8, sum_of_ids_by_condition(&condition,&x));
        assert_eq!(checklist.pop().unwrap(),GameStat::compare_condition(&condition, &x.pop().unwrap().1));
        assert_eq!(checklist.pop().unwrap(),GameStat::compare_condition(&condition, &x.pop().unwrap().1));
        assert_eq!(checklist.pop().unwrap(),GameStat::compare_condition(&condition, &x.pop().unwrap().1));
        assert_eq!(checklist.pop().unwrap(),GameStat::compare_condition(&condition, &x.pop().unwrap().1));
        assert_eq!(checklist.pop().unwrap(),GameStat::compare_condition(&condition, &x.pop().unwrap().1));


    }

}