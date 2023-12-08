use std::borrow::Borrow;
use std::any::Any;
use std::cell::{Ref, RefCell, RefMut};

use std::cmp::{Ordering};
use std::collections::HashMap;
use std::rc::Rc;
use itertools::{Itertools, unfold};
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
        node.key = String::from(kv.0);
        node.left = String::from(lr.0);
        node.right = String::from(lr.1);
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
    let res = ();
    info!("Result :  {:?}" , res);
    println!("Result :  {:?}" , res)
}

pub fn run_day_8_part_2() {
    let  input = include_str!("./input.txt");
    let rounds = from_input_part_2(input) ;

    let res = ();

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
    let starting_node = &nodes[0].key.clone();
    let mut nodes_map: HashMap<&str, &Node> = HashMap::from_iter(nodes.iter().map(|n|(n.key.as_str(), n)));


    let res = unfold((0usize, starting_node.clone()),  |(count, node)| {
        let n = nodes_map.get();
        debug!("Unfold Tuple : {:?} | n = {:?}" , (count,&node) , n );
        if n.key == "ZZZ" {
            debug!("{} == ZZZ", n.key);
            None

        } else
        {
            debug!("{} != ZZZ", n.key);
            Some((*count + 1, n.get_node_from_command(&commands[*count % commands.len() ])))

        }
    });


    res.take(5).last().unwrap().0


}
pub fn from_input_part_2(input : &str ) -> usize {
    let res = ();
    debug!("{:?}",res);
    todo!("PART 2")
}

#[cfg(test)]
mod tests {
    use std::sync::Once;
    use tracing::{info};
    use crate::day8::day8::from_input_part_1;

    const INIT : Once = Once::new();

    pub fn init_logger(){
            INIT.call_once(||tracing_subscriber::fmt::init())
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
        let  input = include_str!("./testInput1.txt");
        let secondary = include_str!("./testInput2.txt");



    }

}