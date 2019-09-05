use std::fs::File;
use std::io::Read;

fn jump(puzzle_input:&str) ->i32{
    let mut current_position: i32 = 0;
    let mut tokens:Vec<i32> = puzzle_input.split_whitespace()
        .map(|num_str| num_str.parse::<i32>().unwrap()).collect();
    
    let mut step_counter:i32 = 0;

    loop {
        match tokens.get_mut(current_position as usize){
            Some(i) => {
                step_counter += 1;
                current_position += *i;
                if *i >= 3 {
                    *i -= 1;  
                } else {
                    *i += 1;
                }
            },
            None => return step_counter,
        }
    }
}
fn main() {
    let mut file = File::open("input.txt").expect("Error opening");
    let mut tokens = String::new();
    file.read_to_string(&mut tokens).expect("error reading");

    let token = "0 3 0 1 -3";
    println!("{}", jump(&tokens));
}
