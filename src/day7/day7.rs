use std::any::Any;

use std::cmp::{Ordering};
use std::collections::HashMap;




use itertools::{Itertools};

use tracing::{debug, info};

use crate::day7::day7::Combination::{FiveOfAKind, FourOfAKind, FullHouse, HighCard, OnePair, ThreeOfAKind, TwoPairs};

#[derive(Debug,Clone,Hash,Eq, PartialEq,Ord, PartialOrd)]
enum Cards {
    A,
    K,
    Q,
    J,
    T,
Nine,
Eight,
Seven,
Six,
Five,
Four,
Three,
Two,
}

impl Cards {
    pub fn value(&self) -> usize {
        match self {
            Cards::A => {8192}
            Cards::K => {4096}
            Cards::Q => {2048}
            Cards::J => {1024}
            Cards::T => {512}
            Cards::Nine => {256}
            Cards::Eight => {128}
            Cards::Seven => {64}
            Cards::Six => {32}
            Cards::Five => {16}
            Cards::Four => {8}
            Cards::Three => {4}
            Cards::Two => {2}
        }
    }
    pub fn value_joker(&self) -> usize {
        match self {
            Cards::A => {8192}
            Cards::K => {4096}
            Cards::Q => {2048}
            Cards::T => {1024}
            Cards::Nine => {512}
            Cards::Eight => {256}
            Cards::Seven => {128}
            Cards::Six => {64}
            Cards::Five => {32}
            Cards::Four => {16}
            Cards::Three => {8}
            Cards::Two => {4}
            Cards::J => {2}
        }
    }

}

impl TryFrom<char> for Cards {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'K' => { Ok(Cards::K) }
            'A' => { Ok(Cards::A) }
            'Q' => { Ok(Cards::Q) }
            'J' => { Ok(Cards::J) }
            'T' => { Ok(Cards::T) }
            '9' => { Ok(Cards::Nine) }
            '8' => { Ok(Cards::Eight) }
            '7' => { Ok(Cards::Seven) }
            '6' => { Ok(Cards::Six) }
            '5' => { Ok(Cards::Five) }
            '4' => { Ok(Cards::Four) }
            '3' => { Ok(Cards::Three) }
            '2' => { Ok(Cards::Two) }
            _ => Err(())
        }
    }
}
impl Cards {

    pub fn slice_5(v :&[Self]) -> Option<[Cards;5]> {
        if v.len() >= 5 {
            Some([v[0].clone(),v[1].clone(),v[2].clone(),v[3].clone(),v[4].clone()])
        }else { None }
    }
}



#[derive(Debug,Clone,Eq,Ord)]
enum Combination {
    FiveOfAKind([Cards;5]),
    FourOfAKind([Cards;5]),
    FullHouse([Cards;5]),
    ThreeOfAKind([Cards;5]),
    TwoPairs([Cards;5]),
    OnePair([Cards;5]),
    HighCard([Cards;5])
}


fn compare_card_slice(left : &[Cards;5] ,other : &[Cards;5]) -> Option<Ordering>{
    debug!("l:{:?}, r:{:?}",left,other);
    let res = left.iter()
        .zip(other)
        .fold(None,|acc,(a,b)|{
            debug!("l:{:?}, r:{:?}",a,b);
            if let Some(o) = acc {
                if o == Ordering::Equal {
                    debug!("Was EQ");
                    if a.value() == b.value(){
                        Some(Ordering::Equal)
                    } else if a.value()> b.value() {
                        Some(Ordering::Greater)
                    } else { Some(Ordering::Less) }
                }else { acc }
            }
            else if a.value()==b.value(){
                Some(Ordering::Equal)
            } else if a.value()>b.value() {
                Some(Ordering::Greater)
            } else { Some(Ordering::Less) }
        });
        debug!("Got {:?}" ,res);
    res
}




fn compare_card_slice_joker(left : &[Cards;5] ,other : &[Cards;5]) -> Option<Ordering>{
    debug!("l:{:?}, r:{:?}",left,other);
    let res = left.iter()
        .zip(other)
        .fold(None,|acc,(a,b)|{
            debug!("l:{:?}, r:{:?}",a,b);
            if let Some(o) = acc {
                if o == Ordering::Equal {
                    debug!("Was EQ");
                    if a.value_joker() == b.value_joker(){
                        Some(Ordering::Equal)
                    } else if a.value_joker()> b.value_joker() {
                        Some(Ordering::Greater)
                    } else { Some(Ordering::Less) }
                }else { acc }
            }
            else if a.value_joker()==b.value_joker(){
                Some(Ordering::Equal)
            } else if a.value_joker()>b.value_joker() {
                Some(Ordering::Greater)
            } else { Some(Ordering::Less) }
        });
    debug!("Got {:?}" ,res);
    res
}



impl Combination {

    fn from_cards(cards : &[Cards]) -> Combination{
        let tmp = cards.iter()
            .counts();
        let card_map: HashMap<&usize,Cards> = tmp
            .iter().map(|(&k,v)| {
            let x =
            (v, k.clone());
            x
        }).collect()
            ;
        let tmp = cards.iter().counts();
        let doubles = tmp.iter().filter(|(&_a,&b)| b==2).clone().collect_vec();
        let c = if let Some(_a) = card_map.get(&5){
            FiveOfAKind(Cards::slice_5(cards).unwrap())
        }else  if let Some(_a) = card_map.get(&4) {
            FourOfAKind(Cards::slice_5(cards).unwrap())
        }else  if let(Some(_a),Some(_b)) = (card_map.get(&3), card_map.get(&2)) {FullHouse(Cards::slice_5(cards).unwrap())}
        else  if let Some(_a) = card_map.get(&3) { ThreeOfAKind(Cards::slice_5(cards).unwrap()) }
            else if doubles.len()== 2 {TwoPairs(Cards::slice_5(cards).unwrap())}
            else if doubles.len()==1 {OnePair(Cards::slice_5(cards).unwrap())}
        else { HighCard(Cards::slice_5(cards).unwrap()) };
        debug!("Combination {:?}",c);
        c
    }

    fn from_cards_joker(cards : &[Cards]) -> JokerCombination{
        let tmp = Self::from_cards(cards);

        let jokers =cards.iter().filter(|e| e.value() == Cards::J.value()).count();
        if jokers == 0 { return JokerCombination(tmp); }


        let c =
        match &tmp{
            FiveOfAKind(a) => {FiveOfAKind(a.clone())}
            FourOfAKind(a) => {if jokers == 1 || jokers == 4 {FiveOfAKind(a.clone())} else {tmp} }
            FullHouse(a) => {if jokers == 3 || jokers == 2 { FiveOfAKind(a.clone())} else if jokers == 1 {FourOfAKind(a.clone())} else { tmp } }
            ThreeOfAKind(a) => {if jokers == 3 {FourOfAKind(a.clone())} else if jokers == 2 {FiveOfAKind(a.clone())} else { FourOfAKind(a.clone()) }  }
            TwoPairs(a) => {if jokers == 2 {FourOfAKind(a.clone())} else {FullHouse(a.clone())}}
            OnePair(a) => {ThreeOfAKind(a.clone()) }
            HighCard(a) => {
                OnePair(a.clone())
            }
        };

        debug!("Combination {:?}",c);
        JokerCombination(c)
    }

    fn self_to_number(&self) -> i32 {
        match self {
            FiveOfAKind(_) => {7}
            FourOfAKind(_) => {6}
            FullHouse(_) => {5}
            ThreeOfAKind(_) => {4}
            TwoPairs(_) => {3}
            OnePair(_) => {2}
            HighCard(_) => {1}
        }
    }

    pub fn get_inner(&self) -> &[Cards;5] {
        match self {
            FiveOfAKind(a) => {a}
            FourOfAKind(a) => {a}
            FullHouse(a) => {a}
            ThreeOfAKind(a) => {a}
            TwoPairs(a) => {a}
            OnePair(a) => {a}
            HighCard(a) => {a}
        }
    }






}
impl PartialEq for Combination {
    fn eq(&self, other: &Self) -> bool {
        debug!("{:?} , {:?}", self,other );
        debug!("{:?} , {:?}", self.type_id(),other.type_id() );
        if self.self_to_number() == other.self_to_number() {
            true
        }else {false}
    }
}

#[derive(Eq,Debug,Ord)]
struct JokerCombination(Combination);

impl PartialOrd for JokerCombination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0.self_to_number() == other.0.self_to_number() {
            compare_card_slice_joker(self.0.get_inner(), other.0.get_inner())
        }else if self.0.self_to_number() > other.0.self_to_number() {
            Some(Ordering::Greater)
        } else { Some(Ordering::Less) }
    }
}
impl PartialEq for JokerCombination {
    fn eq(&self, other: &Self) -> bool {
        debug!("{:?} , {:?}", self,other );
        debug!("{:?} , {:?}", self.type_id(),other.type_id() );
        if self.0.self_to_number() == other.0.self_to_number() {
            true
        }else {false}
    }
}


impl PartialOrd for Combination {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.self_to_number() == other.self_to_number() {
            compare_card_slice(self.get_inner(), other.get_inner())
        }else if self.self_to_number() > other.self_to_number() {
                Some(Ordering::Greater)
            } else { Some(Ordering::Less) }
        }
}





#[derive(Debug,Clone)]
pub struct Round {
    hand : Vec<Cards>,
    bid : u64
}



pub fn run_day_7_part_1() {
    let  input = include_str!("./input.txt");
    let rounds = from_input_part_1(input);

    let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid);

    info!("Result :  {:?}" , res);
    println!("Result :  {:?}" , res)
}

pub fn run_day_7_part_2() {
    let  input = include_str!("./input.txt");
    let rounds = from_input_part_2(input) ;

    let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> Vec<Round> {
    let mut res  = input
        .lines()
        .map(|l|{
            let round = l.split_whitespace()
                .collect_vec();
            let hand = round.first()
                .unwrap()
                .chars()
                .flat_map(|c|Cards::try_from(c).ok()).collect_vec();

            let bid = round.last().unwrap().parse::<u64>().unwrap();
            Round{hand,bid}
        })
        .collect_vec()
;
    res.sort_by_key(|r| {Combination::from_cards(&r.hand)});

    debug!("{:?}",res);


    res

}
pub fn from_input_part_2(input : &str ) -> Vec<Round> {
    let mut res  = input
        .lines()
        .map(|l|{
            let round = l.split_whitespace()
                .collect_vec();
            let hand = round.first()
                .unwrap()
                .chars()
                .flat_map(|c|
                    Cards::try_from(c).ok()).collect_vec();
            let bid = round.last().unwrap().parse::<u64>().unwrap();
            Round{hand,bid}
        })
        .collect_vec()

        ;
    res.sort_by_key(|r| {let x =
        Combination::from_cards_joker(&r.hand);
        x
    }
    );
    debug!("{:?}",res);
    res
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    
    use crate::day7::day7::{Combination, from_input_part_1, from_input_part_2};

    const INIT : Once = Once::new();

    pub fn init_logger(){
        INIT.call_once(||tracing_subscriber::fmt::init())
    }

    #[test]
    fn test_day_7_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");
        let rounds = from_input_part_1(input);
        let rounds2 = from_input_part_1(secondary);
        let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid );
        let res2 = rounds2.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid );

        assert_eq!(6440,res);
        assert_eq!(3542,res2);

    }
    #[test]
    fn test_day_7_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");
        let rounds = from_input_part_2(input);
        let rounds2 = from_input_part_2(secondary);


        let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid );
        let res2 = rounds2.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid );
        info!("===========");
        rounds2.iter()
            .for_each(|a|{
                info!("{:?} {:}",Combination::from_cards_joker(&a.hand).0 , a.bid )
            });

        assert_eq!(5905,res);
        assert_eq!(3667,res2);


    }

}