use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn color_to_idx(color: &str) -> usize {
    match color {
        "red" => 0,
        "green" => 1,
        "blue" => 2,
        _ => 0
    }
}

fn validate_set(set: &Vec<u32>, bag: &Vec<u32>) -> bool {
    for (i,s) in set.iter().enumerate() {
        if bag[i] < *s { return false; }
    }
    return true;
}

fn update_max_set(max_set: &mut Vec<u32>, set: &Vec<u32>) {
    for (i,s) in set.iter().enumerate() {
        if max_set[i] < *s { max_set[i] = *s; }
    }
}

fn part_1(filename: &str){
    let bag = vec![12,13,14];
    let lines = read_file(filename);
    let mut sum = 0;
    for line in lines {
        let mut set_invalid = false;
        let split: Vec<&str> = line.split_whitespace().collect();        
        let game_id = split[1][..split[1].len()-1].parse::<u32>().unwrap();
        let mut set = vec![0,0,0];
        let mut num: u32 = 0;
        for i in 2..split.len() {
            let item = split[i];
            if i % 2 == 0 {
                num = item.parse::<u32>().unwrap_or(0);       
            } else {
                let set_ends = item.contains(';') || i == split.len() - 1;
                let c = if item.contains(';') || item.contains(',') { item[..item.len()-1].as_ref() } else { item };
                set[color_to_idx(c)] = num;                
                if set_ends && !validate_set(&set,&bag) { set_invalid = true; break; }                
                if set_ends { set = vec![0,0,0]; };
            }            
        }
        if !set_invalid { sum += game_id; }
    }
    println!("Answer for part 1 is {}", sum);
}

fn part_2(filename: &str){
    let lines = read_file(filename);
    let mut sum = 0;
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();        
        let mut set = vec![0,0,0];
        let mut max_set = vec![0,0,0];
        let mut num: u32 = 0;
        for i in 2..split.len() {
            let item = split[i];
            if i % 2 == 0 {
                num = item.parse::<u32>().unwrap_or(0);       
            } else {
                let set_ends = item.contains(';') || i == split.len() - 1;
                let c = if item.contains(';') || item.contains(',') { item[..item.len()-1].as_ref() } else { item };
                set[color_to_idx(c)] = num;                
                if set_ends { 
                    update_max_set(&mut max_set, &set);
                    set = vec![0,0,0]; 
                }                
            }            
        }
        sum += max_set.iter().fold(1,|acc,x| acc * x);
    }
    println!("Answer for part 2 is {}", sum);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
