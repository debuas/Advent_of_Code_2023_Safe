use tracing::info;

pub fn run_day_5_part1(){
    let input = include_str!("testinput1.txt");
    let res = from_input_part1(input);

    info!("Result = '{}'", res);
    println!("Result = '{}'", res);
}


pub fn run_day_5_part2(){
    let input = include_str!("testinput1.txt");
    let res = from_input_part2(input);
    info!("Result = '{}'", res);
    println!("Result = '{}'", res);
}

fn from_input_part1(input :&str) -> &'static str {
""
}

fn from_input_part2(input :&str) -> &'static str {
""
}


#[cfg(test)]
mod tests {
    use crate::day5::day5::{from_input_part1, from_input_part2};

    #[test]
    fn test_part1(){
        let input = include_str!("testinput1.txt");
        let res = from_input_part1(input);
    }

    #[test]
    fn test_part2(){
        let input = include_str!("testinput1.txt");
        let res = from_input_part2(input);
    }

}