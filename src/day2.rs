use std::io::Read;
use std::fs::File;
use std::io::{BufReader, Result};

fn difference(puzzle_input:String) -> u32 {
    let mut total = 0;
    puzzle_input.lines().fold(0, |acc, line| {
        let mut values:Vec<u32> = line.split_whitespace().map(|number_string|{ 
            number_string.parse::<u32>().expect("Unable to parse")}).collect();
        values.sort();
        let largest_value = values.last().unwrap();
        let smallest_value = values.first().unwrap();
        let diff = largest_value - smallest_value;
        acc + diff
    })
} 

fn main(){
    let mut file = File::open("inputday2.txt")
        .expect("Unable to open file.");
    let mut puzzle_input = String::new();
    file.read_to_string(&mut puzzle_input)
        .expect("Unable to read the file");
    println!("{}", difference(puzzle_input));
}
