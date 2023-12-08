use std::fs;
use num_integer::lcm;

struct Node{
    l: u32,
    r: u32,
    first: bool,
    last: bool,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str, mult : bool) -> (String,Vec<Node>) {
    let lines = read_file(filename);
    let instr = lines[0].clone();
    let mut tmp: Vec<(&str, &str, &str)> = vec![];
    for line in lines.iter().skip(2) {
        let v: Vec<&str> = line.split_whitespace().collect();
        tmp.push((v[0], &v[2][1..v[2].len()-1], &v[3][..v[3].len()-1]));
    }
    let mut res: Vec<Node> = vec![];
    for t in &tmp {
        res.push(Node { l: tmp.iter().position(|x| x.0 == t.1).unwrap() as u32, 
                        r: tmp.iter().position(|x| x.0 == t.2).unwrap() as u32,
                        first: if !mult { t.0 == "AAA" } else { t.0.as_bytes()[2] as char == 'A' },
                        last:  if !mult { t.0 == "ZZZ" } else { t.0.as_bytes()[2] as char == 'Z' } })
    }
    (instr,res)
}

fn num_steps(first: usize, nodes: &Vec<Node>, instr: &str) -> u32{
    let mut found = false;
    let mut steps = 0;
    let mut curr = first;
    while !found {
        for c in instr.chars() {
            if nodes[curr].last {
                found = true;
                break;
            }            
            curr = match c {
                'L' => nodes[curr].l as usize,
                'R' => nodes[curr].r as usize,
                _ => panic!("Oops")
            };
            steps += 1;
        }
    }
    steps
}

fn part_1(filename: &str) {
    let (instr,nodes) = parse(filename, false);
    let curr = nodes.iter().position(|x| x.first).unwrap();    
    println!("Answer for part 1: {}",num_steps(curr as usize, &nodes, &instr));
}

fn part_2(filename: &str) {
    let (instr,nodes) = parse(filename, true);
    let first: Vec<u32> = nodes.iter()
                            .enumerate()
                            .filter(|(_,x)| x.first )
                            .map(|(i,_)| i as u32 )
                            .collect();
    let steps: Vec<u32> = first.iter().map(|x| num_steps(*x as usize, &nodes, &instr)).collect();
    let m: u64 = steps.iter().fold(1, |a,x| lcm(a,*x as u64));
    println!("Answer for part 2: {}",m);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
