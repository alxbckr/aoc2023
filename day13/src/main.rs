use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) {
}

fn part_1(filename: &str) {
    println!("Answer for part 1: {}",0);
}

fn part_2(filename: &str) {
    println!("Answer for part 2: {}",0);
}

fn main() {
    part_1("puzzle_sample.txt");
    part_2("puzzle_sample.txt");
}
