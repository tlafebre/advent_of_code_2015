use std::fs;

mod day_1;

fn main() {
    let filename = "data/day_1.txt";
    let instructions = fs::read_to_string(filename).unwrap();

    println!("{}", day_1::find_floor(&instructions));
    println!("{}", day_1::find_basement_instruction(&instructions));
    
}
