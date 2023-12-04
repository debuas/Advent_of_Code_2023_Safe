
use tracing::{debug, info};


pub struct Mappings<'a> (char, &'a str);
pub fn run_day1_part_2() {
    info!("Running day 1 , Part 2");
    let input = include_str!("./input1.txt");
    info!("Loaded Input File, lines : {}", input.lines().count());
    info!("Loaded Input File, Total String size : {}", input.len());
    let number = solve_from_input_file(input);
    info!("Result = '{}'", number);
    println!("Result = '{}'", number);

}

fn solve_from_input_file(input : &str) -> u32 {
    input
        .lines()
        .fold(0,|acc,l|{
            debug!("Accumulator: {}",acc);
            acc + to_calibration_values(&convert_string_to_numbers(l)).unwrap_or(0)
        })
}

fn to_calibration_values(input : &[u32]) -> Option<u32> {
    debug!("Values of Input {:?}",input);
    if let (Some(first),Some(last)) = (input.first() , input.last()) {
        Some(first*10+last)
    } else { None }


}
fn convert_string_to_numbers( input : &str) -> Vec<u32>{
    let mapping : [Mappings;9] = [Mappings('1', "one"), Mappings('2', "two"), Mappings('3', "three"), Mappings('4', "four"), Mappings('5', "five"),
        Mappings('6', "six"), Mappings('7', "seven"), Mappings('8', "eight"), Mappings('9', "nine")];

    let mut nums: Vec<u32> = vec![];

    input
        .chars()
        .fold(String::from(""), |acc,c|{
            let combined = format!("{}{}",acc,c);
            //check if current is number , If true Reset to "" and put to Vec else continue
            if let Some(digit) = c.to_digit(10) {
                nums.push(digit);
                "".to_string()
            }
            //check Compound If true put into Vec reset accumulator to current character
            else if let Some(digit) = mapping.iter().find_map(|map| if combined.contains(map.1) { map.0.to_digit(10) } else { None } ) {
                nums.push(digit);
                c.to_string()
            }
            //else combined input
            else {
                combined
            }
        });



    debug!("Converted:'{}' to '{:?}'",input ,nums);
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_2() {
        let input = include_str!("./testInput2.txt");
        let res = solve_from_input_file(input);

        assert_eq!(281,res)
    }

}