use std::borrow::Borrow;
use std::any::Any;


use itertools::{Itertools, unfold};


use tracing::{debug, info, instrument};
use tracing::field::debug;

#[derive(Debug,Clone)]
struct DataSet {
    data_set: Vec<isize>,
    layer: usize,
    lowest: bool
}

impl DataSet {

    pub fn from_line(input : &str) -> Self{
        let data_set = input.split_whitespace().flat_map(|v|v.parse::<isize>()).collect_vec();
        Self {
            data_set,
            layer : 0,
            lowest: false
        }
    }

    pub fn calculate_next_value<'a>(&'a self) -> isize {
        //get next Layer
        let mut buffer: Vec<isize> = Vec::new();

        //pass last to buffer Vec eg 45 -> 15 -> 6 -> 2 -> 0 [shown every step from layer 0]
        let iterator = unfold(self.clone(),|acc|{
            buffer.push(*acc.data_set.last().unwrap_or(&0));
            let new_set = acc.get_next_layer();
            if new_set.lowest {None}
            else {
                *acc = new_set;
                Some(acc.clone())
            }
        });

        //do until reach lowest layer (Zero Layer)
        iterator.last();
        //Reverse List , iterate via fold , acc starts at 0 , acc + next
        buffer.iter().rev().fold(0,|acc,v|acc+v)

    }

    pub fn calculate_previous_value<'a>(&'a self) -> isize {
        //get next Layer
        let mut buffer: Vec<isize> = Vec::new();
        //pass last to buffer Vec eg 45 -> 15 -> 6 -> 2 -> 0 [shown every step from layer 0]
        let iterator = unfold(self.clone(),|acc|{

            buffer.push(*acc.data_set.first().expect("There should be a value"));
            let new_set = acc.get_next_layer();
            if new_set.lowest {None}
            else {
                *acc = new_set;
                Some(acc.clone())
            }
        });

        //do until reach lowest layer (Zero Layer)
        iterator.last();
        //Reverse List , iterate via fold , acc starts at 0 , acc + next
        buffer.iter().rev().fold(0,|acc,v|v-acc)

    }


    pub fn get_next_layer(&self) -> Self{
        let data_set = self.data_set.windows(2).map(|l|l[1]-l[0] ).collect_vec();

        let zero_length =data_set.iter().filter(|&e|e== &0).count();
        if data_set.len() == zero_length || data_set.is_empty()
        { Self { data_set, layer: self.layer + 1, lowest: true } }
        else { Self { data_set, layer: self.layer + 1, lowest: false } }

    }

}





pub fn run_day_9_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);
    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 1:  {:?}" , res)
}

pub fn run_day_9_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input);
    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> isize {
    let data_sets =input
        .lines()
        .map(DataSet::from_line)
        .collect_vec();
    let result = data_sets
        .iter()
        .map(|d| {
            let res = d.calculate_next_value();
            debug!("next Value {} in {:?}", res,d);
            res
        })
        .sum::<isize>()
        ;

    result


}


pub fn from_input_part_2(input : &str ) -> isize  {
    println!("SHOULD RUN PART 2");
    let data_sets =input
        .lines()
        .map(DataSet::from_line)
        .collect_vec();
    debug!("DATA_SETS : {:?}",data_sets);

    let result = data_sets
        .iter()
        .map(|d| {
            let res = d.calculate_previous_value();
            debug!("previous Value {} in {:?}", res,d);
            res
        })
        .sum::<isize>()
        ;

    result
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    use crate::day9::day9::{from_input_part_1, from_input_part_2};

    static INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(tracing_subscriber::fmt::init);
    }

    #[test]
    fn test_day_9_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(114,res);


    }
    #[test]
    fn test_day_9_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_2(input);

        assert_eq!(2,res);
    }

}