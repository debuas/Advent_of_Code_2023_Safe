use std::borrow::Borrow;
use std::any::Any;
use std::collections::{BTreeMap, HashMap};

use itertools::{Itertools, unfold};


use tracing::{debug, info, warn};
use tracing::field::debug;


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
#[derive(Default,Clone,Debug,Eq, PartialEq)]
struct Node {
    pub key : String,
    left : String,
    right : String
}

impl Node {

    pub fn from_line(line : &str) -> Self {
        debug!("{:?}" , line.split('=').collect_vec());
        let kv = line.split('=')
            .collect_tuple::<(&str,&str)>().expect("Should always be size 2 outer Node")

        ;
        let binding =  kv.1
            .replace('(',"").replace(')',"");
        let lr = binding
            .split(',').collect_tuple::<(&str,&str)>().expect("Left right should always be 2 in Nodes")
        ;

        let mut node = Node::default();
        node.key = String::from(kv.0.replace(" ",""));
        node.left = String::from(lr.0.replace(" ",""));
        node.right = String::from(lr.1.replace(" ",""));
        node
    }


    pub fn get_node_from_command(&self, command: &Command) -> &str {
        match command {
            Command::Left => {&self.left}
            Command::Right => {&self.right}
        }
    }

}





pub fn run_day_8_part_1() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_1(input);
    info!("Result Part 1 :  {:?}" , res);
    println!("Result Part 2:  {:?}" , res)
}

pub fn run_day_8_part_2() {
    let  input = include_str!("./input.txt");
    let res = from_input_part_2(input);
    info!("Result Part 2 :  {:?}" , res);
    println!("Result Part 2 :  {:?}" , res)
}



pub fn from_input_part_1(input : &str ) -> usize {
    let mut lines =
    input
        .lines()
        .collect_vec();
    lines.reverse();
    let commands = lines.pop().unwrap().chars().flat_map(|c| { Command::try_from(c).ok() }).collect_vec();
    lines.pop();
    lines.reverse();
    let mut nodes = lines.iter()
        .map(|&l| Node::from_line(l))
        .collect_vec();

    let nodes_map = BTreeMap::from_iter(nodes.iter().map(|n| { (n.key.clone(), n) }));
    let starting_node = &nodes_map["AAA"].key.clone();
    warn!("Test Warning");
    info!("Node counts :{}" , nodes_map.len());
    let mut res = unfold((0usize, starting_node.clone(),0usize),  |(count, node,cycle_count)| {
        let ne = nodes_map.get(node).unwrap();
        println!("Unfold Tuple : {:?} | n = {:?}" , (&count,&node) , ne );
        if ne.key.as_str() == starting_node.as_str() {
            *cycle_count += 1;
            warn!("CICLE DETECTED : {} , Key1: {} Key2: {}" , *cycle_count,ne.key,&starting_node);
        }
        if ne.key == "ZZZ" {
            debug!("{} == ZZZ", ne.key);
            None
        } else {
            debug!("{} != ZZZ", ne.key);
            let res = (*count + 1usize, ne.get_node_from_command(&commands[*count % commands.len() ]));
            *count = res.0;
            *node = res.1.to_string();

            Some(res)
        }
    });


    let x  = res.last().unwrap();
    debug!("{:?}",x);
    x.0
}

fn calculate_min_end_amount_for_node<'a>(starting_node: &'a str, commands: &Vec<Command>, map : &BTreeMap<String,&Node>,) -> (usize, String) {
    let mut res = unfold((0usize, starting_node.clone().to_string(),0usize),  |(count, node,cycle_count)| {
        let ne = map.get(node).unwrap();
        println!("Unfold Tuple : {:?} | n = {:?}", (&count, &node), ne);
        if ne.key.as_str() == starting_node {
            *cycle_count += 1;
            warn!("CICLE DETECTED : {} , Key1: {} Key2: {}" , *cycle_count,ne.key,&starting_node);
        }
        if ne.key.ends_with('Z'){
            debug!("{} ends with Z", ne.key);
            None
        } else {
            debug!("{} does not end with Z", ne.key);
            let res = (*count + 1usize, ne.get_node_from_command(&commands[*count % commands.len()]).to_string());
            *count = res.0;
            *node = res.1.to_string();

            Some(res)
        }
    });
    res.last().unwrap()
}


pub fn from_input_part_2(input : &str ) -> usize {
    let mut lines =
        input
            .lines()
            .collect_vec();
    lines.reverse();
    let commands = lines.pop().unwrap().chars().flat_map(|c| { Command::try_from(c).ok() }).collect_vec();
    lines.pop();
    lines.reverse();
    let mut nodes = lines.iter()
        .map(|&l| Node::from_line(l))
        .collect_vec();

    let nodes_map = BTreeMap::from_iter(nodes.iter().map(|n| { (n.key.clone(), n) }));
    let starting_nodes = &nodes_map.iter().filter(|(k,&n)|n.key.ends_with('A')).map(|(k,n)|n.key.as_str()).collect_vec();
    warn!("Test Warning");
    info!("Node counts :{}" , nodes_map.len());
    //result count, nodes

    let kgv = starting_nodes.iter()
        .map(|&n|calculate_min_end_amount_for_node(n,&commands,&nodes_map))
        .map(|(c,n)| {
            c
        })
        .map(|c|c/commands.len())
        .product::<usize>();

    kgv * commands.len()

}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    use crate::day8::day8::{from_input_part_1, from_input_part_2};

    static INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(tracing_subscriber::fmt::init);
    }

    #[test]
    fn test_day_8_part_1(){
        init_logger();
        let  input = include_str!("./testInput1.txt");
        let res = from_input_part_1(input);

        assert_eq!(2,res)


    }
    #[test]
    fn test_day_8_part_2(){
        init_logger();
        let  input = include_str!("./testInput2.txt");
        let res = from_input_part_2(input);

        info!("Manual Optimized for problem")


    }

}