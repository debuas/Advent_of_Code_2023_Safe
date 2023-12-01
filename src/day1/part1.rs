use tracing::info;

pub fn run_part_1() {
    info!("Running day 1 , Part 1");
    let input = include_str!("./input1.txt");
    info!("Loaded Input File, lines : {}", input.lines().count());
    info!("Loaded Input File, Total String size : {}", input.len());
    let number = solve_part_1_util(input);
    info!("Result = '{}'", number);
    println!("Result = '{}'", number);
}

pub fn solve_part_1_util(input: &str) -> u32 {

    input.lines().fold(0, |acc, l| {
        let mut numbers: Vec<u32> = Vec::new();
        l.chars().for_each(|c| {
            if let Some(x) = c.to_digit(10) {
                numbers.push(x)
            };
        });
        let compound = numbers.first().unwrap() * 10 + numbers.last().unwrap();
        acc + compound
    })
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("./testInput.txt");

        assert_eq!(142, solve_part_1_util(input))
    }
}
