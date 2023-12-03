use std::ops::RangeFrom;
#[derive(Debug)]
struct SchematicSegment {
    line1 : String,
    line2 : String,
    line3 : String,
}
#[derive(Debug)]
struct Position<T> {
    pub value: T,
    pub index_y : i32,
    pub index_beg : i32,
    pub index_end : i32,
}
//(x1,x2,y1,y2)
fn intersect<T,A>(rect1:&Position<T>,rect2: &Position<A>) -> bool {

    // Check if one rectangle is to the left of the other
    if rect1.index_end < rect2.index_beg-1 || rect2.index_end+1 < rect1.index_beg {
        return false;
    }

    // Check if one rectangle is above the other
    if rect1.index_y < rect2.index_y - 1 || rect2.index_y < rect1.index_y - 1 {
        return false;
    }
    // If the above conditions are not met, then the rectangles intersect
    true
}


impl SchematicSegment {

    pub fn from_slice(input : &[Option<&str>;3]) -> Self {
        let line1 = input[0].unwrap_or("").to_string();
        let line2 = input[1].unwrap_or("").to_string();
        let line3 = input[2].unwrap_or("").to_string();
        Self{
            line1,
            line2,
            line3,
        }
    }

    pub fn read_part_numbers(&self) -> Vec<u32> {
        let mut number_pos: Vec<Position<u32>> = vec![];
        let mut symbol_pos: Vec<Position<bool>> = vec![];

        let mut number_buf = String::from("");

        let mut mapfn =  |acc : i32 ,c: char,index_y| {
            if c.is_digit(10) {
                number_buf.push(c)
            }else if c != '.' {
                symbol_pos.push(Position{
                    value: true,
                    index_y,
                    index_beg: acc+1,
                    index_end: acc+1,
                })
            }else {
                if let Some(x) = number_buf.parse::<u32>().ok() {
                    number_pos.push(Position{
                        value: x,
                        index_y,
                        index_beg: acc+1-number_buf.len() as i32,
                        index_end: acc,
                    });
                }
                number_buf.clear()
            }
            ;acc+1
        };
        self.line1.chars()
            .fold(0,|acc,c| mapfn(acc,c,1));
        self.line2.chars()
            .fold(0,|acc,c| mapfn(acc,c,2));
        self.line3.chars()
            .fold(0,|acc,c| mapfn(acc,c,3));

        //validate findall number in range yPos-1 -> yPos+1 , x-1 ,x+1

        println!("Positions numbs : {:?}",number_pos);
        println!("Positions chars : {:?}",symbol_pos);

        let part_numbers :Vec<u32> = number_pos
            .iter()
            .filter(|e| {
                let any = &symbol_pos
                    .iter()
                    .any(|p| {
                        intersect(e,p)
                    });
                *any
            })
            .map(|e| e.value).collect();
        part_numbers
    }



}



pub fn run_day_3_part_1(){
    let  input = include_str!("./input.txt");
}

pub fn run_day_3_part_2(){
    let  input = include_str!("./input.txt");
}


fn from_input(input : &str) -> u32{
    let mut buffer : [Option<&str>;3] = [None,None,None];
    let mut schems: Vec<SchematicSegment> = vec![];

    input
        .lines()
        .for_each(|e| {
            buffer[0] =  buffer[1];
            buffer[1] =  buffer[2];
            buffer[2] =  Some(e);
            schems.push(SchematicSegment::from_slice(&buffer))
        });
    let mut entries = schems.
        iter().map
        (|e| {
            let numbs = e.read_part_numbers();
            println!("{:?}", numbs);
            numbs
        } )
        .fold(Vec::new(), |mut acc, e| {
            acc.extend_from_slice(&e);
            acc
        });
    entries.sort();
    println!("Before deduplication : {:?}",entries);
        entries.dedup();
    println!("after deduplication :  {:?}",entries);
    entries.iter().sum()


}


mod tests {
    use crate::day3::day3::from_input;

    #[test]
    fn test_part_1(){
        let  input = include_str!("./testinput1.txt");

        let res = from_input(input);

        assert_eq!(4361,res)
    }

    #[test]
    fn test_part_2(){
        let  input = include_str!("./testinput1.txt");
    }

}