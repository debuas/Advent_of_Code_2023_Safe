use rayon::iter::ParallelIterator;
use itertools::Itertools;
use rayon::prelude::IntoParallelIterator;
use tracing::{debug, info, instrument};

#[derive(Debug,Clone)]
struct RaceData {
    time : Vec<usize>,
    distance : Vec<usize>
}
#[derive(Debug,Clone)]
struct RaceDataCompact{
    races : Vec<Race>
}
impl RaceDataCompact{

    pub fn from_race_data(data : RaceData) -> Self{
        Self{races : Race::from_vectors(data.time,data.distance)}
    }
    fn calculate_record_beat_counts(&self) -> Vec<usize>{
        self.races.as_slice()
            .into_par_iter().map(|e|e.calculate_amount_of_winning_possibilities())
            .collect()
    }

}


impl RaceData {


}
#[derive(Debug,Clone,Copy)]
struct Race{time : usize ,  distance : usize}

impl Race {

    fn from_vectors(time : Vec<usize>,distance : Vec<usize>) -> Vec<Self>{
        time.iter().zip(distance.iter())
            .map(|(&time,&distance)| Race{time,distance})
            .collect_vec()
    }
    pub fn calculate_amount_of_winning_possibilities(&self) -> usize {
        let _min_speed_needed_per_second = self.distance/self.time;
        let charge_time = 0..self.time;
        charge_time.into_par_iter().filter(|x| {
            let remaining = (self.time - x);
            remaining*x>self.distance
        }).count()
    }
}





pub fn run_day_6_part_1() {
    let  input = include_str!("./input.txt");
    let races = from_input_part_1(input) ;

    let times = races.iter().map(|e| e.calculate_record_beat_counts()).collect_vec();
    info!("Timings : {:#?}",times);
    let res = races.first().unwrap().calculate_record_beat_counts().iter().fold(1,|a,b| a*b);

    info!("Result :  {}" , res);
    println!("Result :  {}" , res)
}

pub fn run_day_6_part_2() {
    let  input = include_str!("./input.txt");
    let races = from_input_part_2(input) ;

    let times = races.iter().map(|e| e.calculate_record_beat_counts()).collect_vec();
    info!("Timings : {:#?}",times);
    let res = races.first().unwrap().calculate_record_beat_counts().iter().fold(1,|a,b| a*b);

    info!("Result :  {}" , res);
    println!("Result :  {}" , res)
}



pub fn from_input_part_1(input : &str ) -> Vec<RaceDataCompact> {
    let race = input
        .lines()
        .collect_vec()
        .chunks(2)
        .map(|(a)|{
            let _b = (
            a[0].replace("Time:", "").trim().split_whitespace().flat_map(|e| e.parse::<usize>()).collect_vec(),
            a[1].replace("Distance:","").trim().split_whitespace().flat_map(|e| e.parse::<usize>()).collect_vec()
            );
            RaceData {time: _b.0, distance: _b.1}
        })
        .map(|d|RaceDataCompact::from_race_data(d))
        .collect_vec()
        ;

    debug!("{:?}",race);

    race

}
pub fn from_input_part_2(input : &str ) -> Vec<RaceDataCompact> {
    let race = input
        .lines()
        .collect_vec()
        .chunks(2)
        .map(|(a)|{
            let _b = (
                vec![a[0].replace("Time:", "").replace(' ', "").trim().parse::<usize>().unwrap()],
                vec![a[1].replace("Distance:","").replace(' ', "").trim().parse::<usize>().unwrap()]
            );
            RaceData {time: _b.0, distance: _b.1}
        })
        .map(|d|RaceDataCompact::from_race_data(d))
        .collect_vec()
        ;

    debug!("{:?}",race);

    race

}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use itertools::{assert_equal, Itertools};
    use tracing::debug;
    use tracing::field::debug;
    use crate::day6::day6::{from_input_part_1, from_input_part_2};

    pub fn init_logger(){
            tracing_subscriber::fmt::init()
    }

    #[test]
    fn test_day_3_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");

        let res = from_input_part_1(input);

        let times = res.iter().map(|e| e.calculate_record_beat_counts()).collect_vec();
        debug!("Timings : {:#?}",times);
        let sum_of_fist = res.first().unwrap().calculate_record_beat_counts().iter().fold(1,|a,b| a*b);
        assert_eq!(288,sum_of_fist);

    }
    #[test]
    fn test_day_3_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");

        let res = from_input_part_2(input);

        let times = res.iter().map(|e| e.calculate_record_beat_counts()).collect_vec();
        debug!("Timings : {:#?}",times);
        let sum_of_fist = res.first().unwrap().calculate_record_beat_counts().iter().fold(1,|a,b| a*b);
        assert_eq!(71503,sum_of_fist);
    }

}