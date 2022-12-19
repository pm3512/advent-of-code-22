use std::{env, fs};

mod part1;
mod part2;

fn main() {
    let args: Vec<String> = env::args().collect();
    let default_filename = String::from("inputs/in1.txt");
    let in_filename = &args.get(1).unwrap_or(&default_filename);

    let input = fs::read_to_string(in_filename).unwrap();
    part1::solve(&input);
    part2::solve(&input);
}