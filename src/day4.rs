use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn is_valid(puzzle_input:String) -> u32 {
    let tokens = puzzle_input.lines();
    tokens
        .map(|word|{
            let mut hold = HashSet::new();
            for w in word.split_whitespace(){
                if hold.contains(w){
                    return 0;
                }
                hold.insert(w);
            }
            return 1;
        })
        .sum()
}

fn is_anagram(puzzle_input:String) -> u32 {
    //TODO
}

fn main() {
    let mut file = File::open("input.txt")
        .expect("Unable to open.");
    let mut tokens = String::new();
    file.read_to_string(&mut tokens)
        .expect("Unable to read file");



    println!("{}",is_valid(tokens));
}
