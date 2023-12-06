use rayon::iter::ParallelIterator;
use std::collections::{BTreeMap, HashMap, HashSet};
use std::iter;
use std::ops::{Range, RangeFrom};
use std::slice::Iter;
use std::sync::Arc;
use itertools::Itertools;
use rangemap::RangeMap;
use rayon::prelude::{IntoParallelIterator, IntoParallelRefIterator};
use tracing::{debug, info};

#[derive(Debug,Clone)]
struct TypeMapping {
    destination_range_start : u64,
    source_range_start: u64,
    range_len: u64
}

impl TypeMapping {

    //Key,Value
    pub fn to_range_and_key_function(&self) -> Box<dyn for<'a> Fn(&'a u64) -> u64 + 'static> {

        let range_start = self.source_range_start.clone();
        let destination_start = self.destination_range_start.clone();
        let dyn_fn : Box<dyn for<'a> Fn(&'a u64) -> u64> = Box::new(move |input : &u64| {
            let x = *input - range_start;
            let res = destination_start+ x;
            res
        });
        dyn_fn
    }

    pub fn to_range_and_key_function_arc(&self) -> Arc<dyn for<'a> Fn(&'a u64) -> u64 + 'static> {

        let range_start = self.source_range_start.clone();
        let destination_start = self.destination_range_start.clone();
        let dyn_fn : Arc<dyn for<'a> Fn(&'a u64) -> u64> = Arc::new(move |input : &u64| {
            let x = *input - range_start;
            let res = destination_start+ x;
            res
        });
        dyn_fn
    }

    pub fn source_range(&self) -> Range<u64> {
        Range {start: self.source_range_start as u64, end: (self.source_range_start + self.range_len) as u64 }
    }

}

    pub fn static_key_to_range_function(range: &Range<u64> , dest_range_start : &u64 ,input : &u64) -> u64 {
        if range.contains(input) {
            let x = input - &range.start;
            let res = dest_range_start + x;
            res
        } else { input.clone() }
    }


#[derive(Debug,Default,Clone)]
struct MappingTypes {
    mapping_in : String,
    mapping_out : String,
    mappings : Vec<TypeMapping>
}

#[derive()]
struct TypeMap{
    relates_to : String,
    mappings : Vec<(Range<u64>, Box<dyn for<'a> Fn(&'a u64) -> u64 + 'static>)>
}
impl TypeMap {
    fn from_mapping_types(types: &MappingTypes) -> Self {
        let mappings = types.mappings
            .iter()
            .map(|v| (v.source_range(), v.to_range_and_key_function()))
            .collect_vec();

        Self {
            relates_to: types.mapping_out.clone(),
            mappings,
        }
    }
}
#[derive(Debug,Clone)]
struct TypeMapArc{
    relates_to : String,
    mappings : Vec<(Range<u64>, u64)>
}
impl TypeMapArc {
    fn from_mapping_types(types: &MappingTypes) -> Self {
        let mappings = types.mappings
            .iter()
            .map(|v| (v.source_range(),v.destination_range_start))
            .collect_vec();

        Self {
            relates_to: types.mapping_out.clone(),
            mappings,
        }
    }
}



#[derive(Debug,Default,Clone)]
struct Seeds {
    seeds: Vec<u64>
}
#[derive(Debug,Default,Clone)]
struct SeedRange {
    seeds: Vec<Range<u64>>
}


#[derive(Debug,Clone,Eq, PartialEq)]
struct SeedTable {
    table : HashMap<String,u64>
}
impl SeedTable {
    pub fn new(seed_id : &u64)->Self{
        Self {table: HashMap::from([("seed".to_string(),seed_id.clone())])}
    }
    fn new_static_part1(seed : u64,soil : u64,fertilizer: u64,water: u64,light: u64,temperature: u64,humidity: u64,location : u64 ) -> Self{
        Self{ table: HashMap::from([
                ("seed".to_string(),seed),
                ("soil".to_string(),soil),
                ("fertilizer".to_string(),fertilizer),
                ("water".to_string(),water),
                ("light".to_string(),light),
                ("temperature".to_string(),temperature),
                ("humidity".to_string(),humidity),
                ("location".to_string(),location)])
        }
    }


}


pub fn run_day_5_part1(){
    let input = include_str!("input.txt");
    let res = from_input_part1(input);
    let min = res.iter().min_by_key(|e| e.table["location"]).unwrap().table["location"];
    info!("Result = '{:?}'", min);
    println!("Result = '{:?}'", min);
}


pub fn run_day_5_part2(){
    let input = include_str!("input.txt");
    let res = from_input_part2(input);
    let min = res.iter().min_by_key(|e| e.table["location"]);
    info!("Result = '{:?}'", min);
    println!("Result = '{:?}'", min);
}

fn from_input_part1(input :&str) -> Vec<SeedTable> {
    // Read Mappings´ Seperate into blocks starting with a string, split by ":" numbers next_input_string or blank
    let binding = input
        //Set End For Numbers as markers
        .replace("\n\n", "\nENDOFNUMBERS\n");
    println!("{:?}",&binding);
    let extract = binding
        //need it still as Identifier
        .lines()
        .map(|l|l.split(':').collect_vec())
        .map(|e|e.iter().flat_map( |&i|if i == "" {None} else {Some(i.replace(" map" ,""))} ).collect_vec())
        .collect_vec();
    //
    println!("{:?}",&extract);




    let mut seeds : Seeds = Seeds::default();
    let mut vec_mappings: Vec<MappingTypes> = vec![];

    let mut fun_it = |it:  &'_ mut Iter<Vec<String>> | {
            if let Some(identifier) = it.next() {
                if identifier.first().unwrap() == "seeds" {
                    let seed = identifier.last().unwrap().split_whitespace().flat_map(|e| e.parse::<u64>()).collect_vec();
                    seeds = Seeds { seeds: seed };

                } else if identifier.first().unwrap().contains("-to-") {
                    println!("Jumping into TO");
                    let split = identifier.first().unwrap().split("-to-").collect_vec();
                    let mut new_mapping = MappingTypes {
                        mapping_in: split.first().unwrap().to_string(),
                        mapping_out: split.last().unwrap().to_string(),
                        mappings: vec![],
                    };
                    //iterate till marker
                    while let Some(x) = it.next() {
                        if x.first().unwrap().contains("ENDOFNUMBERS") { break; } else {
                            let maping_vec = x.first().unwrap().split_whitespace().flat_map(|i| i.parse::<u64>()).collect_vec();
                            let type_map = TypeMapping {
                                destination_range_start: maping_vec[0],
                                source_range_start: maping_vec[1],
                                range_len: maping_vec[2],
                            };
                            new_mapping.mappings.push(type_map)
                        }
                    }
                    vec_mappings.push(new_mapping);


                }
                Some(0)
            } else { None }

        };


    let _ = extract.iter()
        .batching(|it| match fun_it(it) {
            None => { None }
            Some(ex) => { fun_it(it) }
        }).collect_vec();

    //println!("{:?}",&seeds);
    //println!("{:?}",&vec_mappings);

    //now Start a Mapping tree
    // Seed Soil Fertilizer Water Light temperature humidity location

    let multimap : HashMap<String,TypeMap>= HashMap::from_iter(
        vec_mappings.iter().map(|types|{
            (
                types.mapping_in.clone(),
                TypeMap::from_mapping_types(types)
            )
        }).collect_vec()
    );
    //println!("MAP : {:?}",multimap);
    let seedTable = seeds.seeds.iter().map(|e|{
        let mut s = SeedTable::new(e);
        let mut index = Some("seed");
        while let Some(key) = index {
            if let Some(table)= multimap.get(key){
                let v = table.mappings
                    .iter()
                    .find_map(|(r,fun)|{
                        if r.contains(&s.table[key]) {
                            Some(fun(&s.table[key]))
                        }else {None}
                    })
                    .unwrap_or(s.table[key]) ;
                s.table.insert(table.relates_to.clone(),v);
                index= Some(&table.relates_to)
            } else {index = None}
        }
        s
    }).collect_vec();

    info!("{:#?}",seedTable);

    seedTable
}







fn from_input_part2(input :&str) -> Vec<SeedTable> {
    info!("RUNNING PART 2");
    // Read Mappings´ Seperate into blocks starting with a string, split by ":" numbers next_input_string or blank
    let binding = input
        //Set End For Numbers as markers
        .replace("\n\n", "\nENDOFNUMBERS\n");
    //println!("{:?}",&binding);
    let extract = binding
        //need it still as Identifier
        .lines()
        .map(|l|l.split(':').collect_vec())
        .map(|e|e.iter().flat_map( |&i|if i == "" {None} else {Some(i.replace(" map" ,""))} ).collect_vec())
        .collect_vec();
    //
    //println!("{:?}",&extract);




    let mut seeds : SeedRange = SeedRange::default();
    let mut vec_mappings: Vec<MappingTypes> = vec![];
    let mut fun_it = |it:  &'_ mut Iter<Vec<String>> | {
        if let Some(identifier) = it.next() {
            if identifier.first().unwrap() == "seeds" {
                let seed  = identifier.last().unwrap()
                    .split_whitespace()
                    .flat_map(|e| {
                    e.parse::<u64>()})
                    .collect_vec()
                    .chunks(2).map(|c| (c[0], c[0] + c[1]))
                    .collect_vec();
                info!("CALC SEED VEV{:?}",seed);
                let mut ranges = Vec::<Range<u64>>::new();

                let ranges = seed.iter()
                    .map(|(a,b)| {
                        info!("{}..{}",a,b);
                        Range { start: *a, end: *b }
                    }).collect_vec();
                seeds.seeds = ranges.clone();
            } else if identifier.first().unwrap().contains("-to-") {
                //println!("Jumping into TO");
                let split = identifier.first().unwrap().split("-to-").collect_vec();
                let mut new_mapping = MappingTypes {
                    mapping_in: split.first().unwrap().to_string(),
                    mapping_out: split.last().unwrap().to_string(),
                    mappings: vec![],
                };
                //iterate till marker
                while let Some(x) = it.next() {
                    if x.first().unwrap().contains("ENDOFNUMBERS") { break; } else {
                        let maping_vec = x.first().unwrap().split_whitespace().flat_map(|i| i.parse::<u64>()).collect_vec();
                        let type_map = TypeMapping {
                            destination_range_start: maping_vec[0],
                            source_range_start: maping_vec[1],
                            range_len: maping_vec[2],
                        };
                        new_mapping.mappings.push(type_map)
                    }
                }
                vec_mappings.push(new_mapping);


            }
            Some(0)
        } else { None }

    };


    let _ = extract.iter()
        .batching(|it| match fun_it(it) {
            None => { None }
            Some(ex) => { fun_it(it) }
        }).collect_vec();

    let multimap : HashMap<String,TypeMapArc>= HashMap::from_iter(
        vec_mappings.iter().map(|types|{
            (
                types.mapping_in.clone(),
                TypeMapArc::from_mapping_types(types)
            )
        }).collect_vec()
    );
    //println!("MAP : {:?}",multimap);
    println!("SEEDS: {:?}", seeds );

    let seedTable = seeds.seeds.iter().filter_map(|e|{
        let count = e.clone().count();
        info!("RUNNING SEED RANGE '{:?}' , LENGTH : '{}'",&e,count);



        e.clone().into_par_iter().map(|e|{
                debug!("processing : {}",e);
                let mut s = SeedTable::new(&e);
                let mut index = Some("seed");
                while let Some(key) = index {
                    if let Some(table)= multimap.get(key){
                        let v = table.mappings
                            .iter()
                            .find_map(|(r,rd)|{
                                if r.contains(&s.table[key]) {
                                    Some(
                                        static_key_to_range_function(r,rd,&s.table[key])
                                        )
                                }else {None}
                            })
                            .unwrap_or(s.table[key]) ;
                        s.table.insert(table.relates_to.clone(),v);
                        index= Some(&table.relates_to)
                    } else {index = None}
                }
                debug!("This is {:?}",s);
                s
            }).min_by_key(|e|e.table["location"])
    })
        .collect_vec();

    info!("{:#?}",seedTable);

    seedTable
}

#[derive(Debug)]
struct RangeSeed {
    source_range : Range<u64>,
    destination_range : Range<u64>
}

struct SeedMap(Vec<RangeSeed>);

fn improved_range_mapper(pre_processed_lines : &str){

}

fn optimize_part_2(input : &str) {
    info!("RUNNING PART 2");
    // Read Mappings´ Seperate into blocks starting with a string, split by ":" numbers next_input_string or blank
    let binding = input
        //Set End For Numbers as markers
        .replace("\n\n", "\nENDOFNUMBERS\n");
    //println!("{:?}",&binding);
    let extract = binding
        //need it still as Identifier
        .lines()
        .map(|l|l.split(':').collect_vec())
        .map(|e|e.iter().flat_map( |&i|if i == "" {None} else {Some(i.replace(" map" ,""))} ).collect_vec())
        .collect_vec();
    //
    //println!("{:?}",&extract);




    let mut seeds : SeedRange = SeedRange::default();
    let mut vec_mappings: Vec<MappingTypes> = vec![];
    let mut fun_it = |it:  &'_ mut Iter<Vec<String>> | {
        if let Some(identifier) = it.next() {
            if identifier.first().unwrap() == "seeds" {
                let seed  = identifier.last().unwrap()
                    .split_whitespace()
                    .flat_map(|e| {
                        e.parse::<u64>()})
                    .collect_vec()
                    .chunks(2).map(|c| (c[0], c[0] + c[1]))
                    .collect_vec();
                info!("CALC SEED VEV{:?}",seed);
                let mut ranges = Vec::<Range<u64>>::new();

                let ranges = seed.iter()
                    .map(|(a,b)| {
                        info!("{}..{}",a,b);
                        Range { start: *a, end: *b }
                    }).collect_vec();
                seeds.seeds = ranges.clone();
            } else if identifier.first().unwrap().contains("-to-") {
                //println!("Jumping into TO");
                let split = identifier.first().unwrap().split("-to-").collect_vec();
                let mut new_mapping = MappingTypes {
                    mapping_in: split.first().unwrap().to_string(),
                    mapping_out: split.last().unwrap().to_string(),
                    mappings: vec![],
                };
                //iterate till marker
                while let Some(x) = it.next() {
                    if x.first().unwrap().contains("ENDOFNUMBERS") { break; } else {
                        let maping_vec = x.first().unwrap().split_whitespace().flat_map(|i| i.parse::<u64>()).collect_vec();
                        let type_map = TypeMapping {
                            destination_range_start: maping_vec[0],
                            source_range_start: maping_vec[1],
                            range_len: maping_vec[2],
                        };
                        new_mapping.mappings.push(type_map)
                    }
                }
                vec_mappings.push(new_mapping);


            }
            Some(0)
        } else { None }

    };
}

#[cfg(test)]
mod tests {
    use assert_unordered::assert_eq_unordered;
    use crate::day5::day5::{from_input_part1, from_input_part2, SeedTable};

    #[test]
    fn test_part1(){
        let input = include_str!("testinput1.txt");
        let res = from_input_part1(input);

        let test_table = vec![
            SeedTable::new_static_part1(79,81,81,81,74,78,78,82),
            SeedTable::new_static_part1(14,14,53,49,42,42,43,43),
            SeedTable::new_static_part1(55,57,57,53,46,82,82,86),
            SeedTable::new_static_part1(13,13,52,41,34,34,35,35),
        ];

        assert_eq_unordered!(&res,&test_table);
        let min = res.iter().min_by_key(|e| e.table["location"]).unwrap().table["location"];
        assert_eq!(min,35)


    }

    #[test]
    fn test_part2(){
        tracing_subscriber::fmt::init();
        let input = include_str!("testinput1.txt");
        let res = from_input_part2(input);
        let min = res.iter().min_by_key(|e| e.table["location"]).unwrap().table["location"];
        println!("Result {:?}",min);
        assert_eq!(min,46)
    }

}