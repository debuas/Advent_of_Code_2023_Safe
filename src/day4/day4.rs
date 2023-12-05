use std::cell::RefCell;
use std::collections::HashMap;
use itertools::Itertools;
use tracing::{debug, info};

pub fn run_day_4_part_1(){
    let  input = include_str!("./input.txt");
    let rounds = from_input_part1(input);
    let mut points = rounds.iter().map(|e|e.first().unwrap().calculate_points(e.last().unwrap())).collect_vec();
    let res : u32 = points.iter().sum();
    info!("Result: {:?}", res);
    println!("Result: {:?}", res)
}pub fn run_day_4_part_2(){
    let  input = include_str!("./input.txt");
    let mut rounds = from_input_part_2(input);
    let res = rounds.iter().fold(0,|acc,(k,e)| acc + e.amount);
    info!("Result: {:?}", res);
    println!("Result: {:?}", res)
}
#[derive(Debug)]
struct Card {
    deck : Vec<u32>,
}



impl Card {

    fn winning_numbers(&self, other : &Card) -> Vec<u32>{
        let wins = self.deck.iter().filter(|e| other.deck.contains(e)).copied().collect_vec();
        wins
    }

    fn calculate_points(&self, other : &Card) -> u32 {
        let numbs = self.winning_numbers(other);
        numbs.iter().fold((1, 0),|mut acc: (u32, u32), e|{
            if acc.0 == 1 {(acc.0+1 , acc.1+1)}
            else { (acc.0+1 , acc.1*2) }

        }).1
    }

}
#[derive(Debug,Clone)]
struct WinsAndScratches {
    wining_cards: usize,
    amount : usize
}

impl WinsAndScratches {
    fn add_amount(&mut self ,amount : usize){
        self.amount = self.amount + amount
    }
}



fn from_input_part1(input :&str) -> Vec<Vec<Card>>{
    let rounds = input
        .lines().map(|e| {
        e.split(':').collect_vec().last().unwrap()
            .split('|').collect_vec().iter()
            .map(|dec| {
                let deck = dec.split(' ').fold(Vec::new(),|mut acc: Vec<u32>,numbs|{
                    if let Some(n) = numbs.parse::<u32>().ok() {acc.push(n)};
                    acc
                });
                debug!("{:?}",deck);
                Card {deck}
            }).collect_vec()

    }).collect_vec();

    rounds
}

fn from_input_part_2<'a>(input : &str) -> Vec<(usize, WinsAndScratches)> {
    let rounds = input
        .lines().map(|e| {
        e.split(':').collect_vec().last().unwrap()
            .split('|').collect_vec().iter()
            .map(|dec| {
                let deck = dec.split(' ').fold(Vec::new(),|mut acc: Vec<u32>,numbs|{
                    if let Some(n) = numbs.parse::<u32>().ok() {acc.push(n)};
                    acc
                });
                debug!("{:?}",deck);
                Card {deck}
            }).collect_vec()

    }).collect_vec();

    //Each with 1 Instance (card|cards with scratches (amount) | wins)
    let mut winning_numbers = rounds.iter().map(|e|e.first().unwrap().winning_numbers(e.last().unwrap()).len())
        .map(|e| WinsAndScratches{ wining_cards: e, amount: 1 })
        .enumerate()
        ;

    //convert numbers to hashmap for each game

    let tracking_cards : RefCell<HashMap<usize,WinsAndScratches>> = RefCell::new(HashMap::from_iter(winning_numbers.clone()))
        ;

    winning_numbers.for_each(|(index,element)| {
        let get_element = &tracking_cards.borrow()[&index].clone();
        let range = index+1..=(index+get_element.wining_cards);
        debug!("Processing: {}",index);
        range.for_each(|e|{

            if let Some(w) = tracking_cards.borrow_mut().get_mut(&e) {
                w.add_amount(get_element.amount);
                debug!("Updating index {} to {}", e,w.amount)
            } else { };
        })
        ;
    });

    let mut tmp = tracking_cards.borrow();
    let mut res = tmp.iter().map(|(k,v)|(k.clone(), v.clone())).collect_vec();
    res.sort_by(|a,b| { a.0.cmp(&b.0) });
    res
}


mod tests {
    use itertools::{assert_equal, Itertools};
    use tracing::{debug, info};
    use crate::day4::day4::{Card, from_input_part1, from_input_part_2};

    #[test]
    fn test_part_1(){
        let input = include_str!("./testInput1.txt");
        let rounds = from_input_part1(input);
        println!("{:?}",rounds);
        let mut winning_numbers = rounds.iter().map(|e|e.first().unwrap().winning_numbers(e.last().unwrap())).collect_vec();
        println!("{:?}",winning_numbers);
        let mut points = rounds.iter().map(|e|e.first().unwrap().calculate_points(e.last().unwrap())).collect_vec();

        assert_eq!(Vec::<u32>::new(),winning_numbers.pop().unwrap());
        assert_eq!(Vec::<u32>::new(),winning_numbers.pop().unwrap());
        assert_eq!(vec![84],winning_numbers.pop().unwrap());
        assert_eq!(vec![1,21],winning_numbers.pop().unwrap());
        assert_eq!(vec![32,61],winning_numbers.pop().unwrap());
        assert_eq!(vec![48,83,86,17],winning_numbers.pop().unwrap());

        let sum = points.iter().sum::<u32>();


        assert_eq!(0,points.pop().unwrap());
        assert_eq!(0,points.pop().unwrap());
        assert_eq!(1,points.pop().unwrap());
        assert_eq!(2,points.pop().unwrap());
        assert_eq!(2,points.pop().unwrap());
        assert_eq!(8,points.pop().unwrap());
        assert_eq!(sum,13);

    }

    #[test]
    fn test_part_2(){
        let input = include_str!("./testInput1.txt");
        let mut rounds = from_input_part_2(input);
        let sum = rounds.iter().fold(0,|acc,(k,e)| acc + e.amount);
        println!("{:?}",rounds);
        println!("{:?}",sum);
        assert_eq!(1,rounds.pop().unwrap().1.amount);
        assert_eq!(14,rounds.pop().unwrap().1.amount);
        assert_eq!(8,rounds.pop().unwrap().1.amount);
        assert_eq!(4,rounds.pop().unwrap().1.amount);
        assert_eq!(2,rounds.pop().unwrap().1.amount);
        assert_eq!(1,rounds.pop().unwrap().1.amount);
        assert_eq!(sum,30);


    }
}
