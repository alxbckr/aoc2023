use std::fs;
use itertools::Itertools;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> (Vec<Vec<char>>,Vec<Vec<u32>>) {
    let mut s: Vec<Vec<char>> = vec![];
    let mut g: Vec<Vec<u32>> = vec![];
    let file = read_file(filename);
    for f in file {
        let spl: Vec<&str> = f.split_whitespace().collect();
        s.push(spl[0].chars().collect());
        g.push(spl[1].split(",").map(|x| x.parse().unwrap()).collect());
    }
    (s,g)
}

fn generate_perm(inp: &Vec<char>) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    let cnt = inp.iter().filter(|&x| *x == '?' ).count();
    let characters = vec![".", "#"];
    let combinations : Vec<_> = (2..cnt).fold(
        characters.iter().map(|c| characters.iter().map(move |&d| d.to_owned() + *c)).flatten().collect(),
        |acc,_| acc.into_iter().map(|c| characters.iter().map(move |&d| d.to_owned() + &*c)).flatten().collect()
    );

    for c in combinations {
        let mut ri = 0;
        let cv: Vec<char> = c.chars().collect();
        let mut line: Vec<char> = vec![];
        for ci in inp {
            if *ci == '?' {
                line.push(cv[ri]);
                ri += 1;
            } else {
                line.push(ci.clone());
            }
        }
        //println!("{:?}",line);        
        res.push(line);
    }
    res
}

fn validate(perm: &Vec<char>, rule: &Vec<u32>) -> bool {
    let mut ci = 0;
    let mut rcnt = 0;
    let mut last_char = '.';
    for p in perm {
        last_char = *p;
        if *p == '#' {
            rcnt += 1;
        } else if rcnt > 0 {
            if ci >= rule.len() || rule[ci] != rcnt {
                return false;
            }
            ci += 1;
            rcnt = 0;
        } 
    }
    last_char == '#' && ci == rule.len() - 1 && rcnt == rule[ci]  
        || last_char == '.' && ci >= rule.len()
}

fn part_1(filename: &str) {
    let mut sum = 0;
    let (lines,g) = parse(filename);
    for (i,line) in lines.iter().enumerate() {
        let perms = generate_perm(line);
        for p in perms{
            if validate(&p, &g[i]) {
                sum += 1;
                //println!("{:?} {:?}",&p, g[i]);
            }
        }
    }
    println!("Answer for part 1: {}",sum);
}

fn part_2(filename: &str) {
    let mut sum = 0;
    let (lines, g) = parse(filename);    
    for (i,line) in lines.iter().enumerate() {
        let mut nl: Vec<char> = vec![];
        let mut ng: Vec<u32> = vec![];
        for i in 0..5 {
            nl.append(&mut line.clone());
            ng.append(&mut g[i].clone());            
        }
        let perms = generate_perm(&nl);
        for p in perms{
            if validate(&p, &ng) {
                sum += 1;
                //println!("{:?} {:?}",&p, g[i]);
            }
        }
    }
    println!("Answer for part 2: {}",sum);
}

fn main() {
    part_1("puzzle_sample.txt");
    part_2("puzzle_sample.txt");
}
