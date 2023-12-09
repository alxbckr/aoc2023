use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<i32>> {
    let mut res = vec![];
    let lines = read_file(filename);
    for line in lines {
        let n = line.split_whitespace().map(|x| x.parse().unwrap()).collect();
        res.push(n);
    }
    res
}

fn produce_hist(nums : &Vec<i32>) -> Vec<Vec<i32>> {
    let mut h: Vec<Vec<i32>> = vec![];
    h.push(nums.clone());
    let mut ready = false;
    while !ready {
        let mut line: Vec<i32> = vec![];        
        ready = true;
        for x in 1..h[h.len()-1].len() {
            let v = h[h.len()-1][x] - h[h.len()-1][x-1];
            line.push(v);
            ready = ready && v == 0;
        }
        h.push(line);
    }
    h
}

fn part_1(filename: &str) {
    let nums = parse(filename);
    let mut ans = 0;
    for n in nums {
        let mut h = produce_hist(&n);

        let mut prev_val = 0;
        for line in h.iter_mut().rev() {
            prev_val += line.last().unwrap();
            line.push(prev_val);
        }

        ans += h[0].last().unwrap();
    }
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    let nums = parse(filename);
    let mut ans = 0;
    for n in nums {
        let mut h = produce_hist(&n);

        let mut prev_val = 0;
        for line in h.iter_mut().rev() {
            prev_val = line.first().unwrap() - prev_val;
            line.push(prev_val);
        }

        ans += h[0].last().unwrap();
    }
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
