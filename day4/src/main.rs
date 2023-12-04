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
        let nums: Vec<&str> = line.split(':').last().unwrap().split('|').collect::<Vec<&str>>();
        let winning_nums: Vec<u32> = nums[0].split(" ").filter(|s| !s.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        let card_nums: Vec<u32> = nums[1].split(" ").filter(|s| !s.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        let mut value = 0;
        for _c in card_nums.iter().filter(|x| winning_nums.contains(x)){
            if value == 0 { value = 1;} else { value *= 2; };
        }
        sum += value;
    }
    println!("Answer for part 1 is {}", sum);
}

fn part_2(filename: &str){   
    let lines = read_file(filename);
    let mut card_inst_counts: Vec<u32> = vec![1;lines.len()];
    for (i,line) in lines.iter().enumerate() {
        let nums: Vec<&str> = line.split(':').last().unwrap().split('|').collect::<Vec<&str>>();
        let winning_nums: Vec<u32> = nums[0].split(" ").filter(|s| !s.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();
        let card_nums: Vec<u32> = nums[1].split(" ").filter(|s| !s.is_empty()).map(|x| x.parse::<u32>().unwrap()).collect();        
        let win_count = card_nums.iter().filter(|x| winning_nums.contains(x)).count();
        for j in i+1..i+1+win_count {
            card_inst_counts[j] += card_inst_counts[i] * 1;
        } 
    }
    println!("Answer for part 2 is {}", card_inst_counts.iter().sum::<u32>());     
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
