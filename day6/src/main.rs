use std::fs;

struct Race {
    time: u64,
    record: u64,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Race> {
    let lines = read_file(filename);
    let times: Vec<u64> = lines[0][10..].split_ascii_whitespace().filter(|y| !y.is_empty()).map(|y| y.parse().unwrap()).collect();
    let dist: Vec<u64> = lines[1][10..].split_ascii_whitespace().filter(|y| !y.is_empty()).map(|y| y.parse().unwrap()).collect();
    let res: Vec<Race> = times.iter().zip(dist.iter()).map(|(x,y)| Race{  time: *x, record: *y }).collect();      
    res
}

fn parse2(filename: &str) -> Race {
    let lines = read_file(filename);
    let times = lines[0][10..].replace(" ", "").parse().unwrap();
    let dist = lines[1][10..].replace(" ", "").parse().unwrap();
    Race { time: times, record: dist }
}

fn part_1(filename: &str) {
    let races = parse(filename);
    let mut answer = 1;
    for r in races{
        let mut num_beats = 0;
        // hold em
        for s in 1..r.time {
            let dist = s * ( r.time - s);
            if dist > r.record { num_beats += 1; };
        }
        answer *= num_beats;
    }
    println!("Answer for part 1: {}",answer);
}

fn part_2(filename: &str) {
    let race = parse2(filename);
    let mut answer = 1;
    let mut num_beats = 0;
    // hold em
    for s in 1..race.time {
        let dist = s * ( race.time - s);
        if dist > race.record { num_beats += 1; };
    }
    answer *= num_beats;
    println!("Answer for part 1: {}",answer);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
