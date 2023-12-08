use std::any::Any;

use std::cmp::{Ordering};
use std::collections::HashMap;
use std::rc::Rc;
use itertools::{Itertools};
use itertools::FoldWhile::Continue;

use tracing::{debug, info};

enum Command {
    Left, Right
}

impl TryFrom<char> for Command{
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' | 'l' => {Ok( Self::Left)},
            'R' | 'r' => {Ok( Self::Right)},
            _ => { Err(()) }
        }
    }
}
#[derive(Default,Clone,Debug)]
struct Node {
    pub key : String,
    kl : String,
    kr : String,
    left : Option<Rc<Self>>,
    right : Option<Rc<Self>>
}

impl Node {

    pub fn from_line(line : &str) -> Self {
        let kv = line.split('=')
            .collect_tuple::<(&str,&str)>().expect("Should always be size 2")

        ;
        let lr = kv.1
            .replace('(',"").replace(')',"")
            .split(',').collect_tuple::<(&str,&str)>().expect("Left right should always be 2")
        ;

        let mut node = Node::default();
        node.key = kv.0.into_string();
        node.kl = lr.0.into_string();
        node.kr = lr.1.into_string();
        node
    }
    pub fn traverse_unit_from_pattern(&self ,pattern : &[Command] ,k : &str){
        let mut current_node = Some(self);
        let mut counter = 0;
        while current_node.key != k  {
            let com = pattern[counter%pattern.len()];
            current_node = current_node.unwrap().get_Node_from_Command()

        }
        ;
    }

    pub fn get_Node_from_Command(&self, command: Command) -> Option<Rc<Node>> {
        match command {
            Command::Left => {self.left.clone()}
            Command::Right => {self.right.clone()}
        }
    }

}





pub fn run_day_8_part_1() {
    let  input = include_str!("./input.txt");
    info!("Result :  {:?}" , res);
    println!("Result :  {:?}" , res)
}

pub fn run_day_8_part_2() {
    let  input = include_str!("./input.txt");
    let rounds = from_input_part_2(input) ;

    let res = rounds.iter().enumerate().into_iter().fold(0 ,|acc,(n,round)| acc+ (n+1) as u64 * round.bid);

    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> Vec<""> {
    let mut lines =
    input
        .lines()
        .collect_vec();
    let commands = lines.pop();
    lines.pop();
    let mut nodes = lines.iter()
        .map(|&l| Rc::new(Node::from_line(l)))
        .collect_vec();
    let starting_node = &nodes[0].key;
    let mut nodes_map = HashMap::from_iter(nodes.iter().map(|n|(n.key.clone(), n)));
    nodes_map.iter_mut().for_each(|(k,n)| {
        let l = nodes_map.get(&n.kl);
        let r = nodes_map.get(&n.kr);
        if let Some(&l) = l {
            n.left = Some(l.clone());
        }
        if let Some(&r) = r {
            n.right = Some(r.clone());
        }
    });




    todo!("PART 1")
}
pub fn from_input_part_2(input : &str ) -> Vec<""> {

    debug!("{:?}",res);
    todo!("PART 2")
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
    fn test_day_8_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");


    }
    #[test]
    fn test_day_8_part_2(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");



    }

}