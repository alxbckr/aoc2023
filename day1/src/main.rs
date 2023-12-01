use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn part_1(filename: &str){
    let lines = read_file(filename);
    let mut sum = 0;
    for line in lines {
        let digits: Vec<u32> = line.chars().filter_map(|a| a.to_digit(10)).collect();
        if digits.len() > 1 {
            sum += digits[0]*10 + digits[digits.len()-1];
        } else {
            sum += digits[0]*10 + digits[0];
        }
    }
    println!("Answer for part 1 is {}", sum);
}

fn part_2(filename: &str){
    let lines = read_file(filename);
    let mut sum = 0;
    let dict = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    for line in lines {
        let mut digits = Vec::new();
        for (i,c) in line.char_indices() {
            if c.is_digit(10) {
                digits.push(c.to_digit(10).unwrap());
            } else {
            }
            for (j,d) in dict.iter().enumerate() {
                if i + d.len() <= line.len() && *d == &line[i..i+d.len()] {
                    digits.push((j + 1) as u32);
                }
            }
        }

        if digits.len() > 1 {
            sum += digits[0]*10 + digits[digits.len()-1];
        } else {
            sum += digits[0]*10 + digits[0];
        }        
    }
    println!("Answer for part 2 is {}", sum);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
