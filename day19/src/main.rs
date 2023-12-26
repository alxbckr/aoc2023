use std::fs;


#[derive(Debug, PartialEq)]
enum OpCode {
    GREATER,
    LESS,
    NONE
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Outcome {
    ACCEPTED,
    REJECTED,
    JUMP
}

#[derive(Debug)]
struct Instruction{
    part_id: Option<usize>,
    oper: OpCode,
    value: usize,
    wf_id : String,
    outcome: Outcome,
}

impl Instruction{
    fn new_unconditional(wf_id: String, outcome: Outcome) -> Self {
        Self { part_id: None, oper: OpCode::NONE, value: 0, wf_id, outcome }
    }
}

struct Workflow{
    id: String,
    instructions: Vec<Instruction>,
}

#[derive(Debug)]
struct Part{
    values: Vec<usize>,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> (Vec<Workflow>,Vec<Part>) {
    let mut wfs = Vec::new();
    let mut parts = Vec::new();
    let mut read_parts = false;
    for line in read_file(filename) {
        if line.is_empty() { 
            read_parts = true;
            continue;
        }
        if read_parts{
            parts.push(Part{values:
                    line[1..line.len()-1].split(",")
                    .map(|spl| spl[2..spl.len()].parse().unwrap()).collect()} );
        } else {
            let id = line[..line.find("{").unwrap()].to_owned();            
            let instr_str: Vec<&str> = line[line.find("{").unwrap()+1..line.len()-1].split(",").collect();
            let mut instr: Vec<Instruction> = Vec::new();
            for i in instr_str {
                if i == "A" { // unconditionally accepted
                    instr.push(Instruction::new_unconditional(String::new(),Outcome::ACCEPTED)); 
                    continue;
                }
                if i == "R" { // unconditionally rejected
                    instr.push(Instruction::new_unconditional(String::new(),Outcome::REJECTED)); 
                    continue;
                }    
                let colon_pos = i.find(":");
                if colon_pos.is_none() { // unconditional jump
                    instr.push(Instruction::new_unconditional(i.to_owned(),Outcome::JUMP)); 
                    continue;                    
                }            
                // condition
                let part_id = match i.chars().nth(0).unwrap() {
                    'x' => 0,
                    'm' => 1,
                    'a' => 2,
                    's' => 3,
                    _ => 0,
                };
                let op = match i.chars().nth(1).unwrap() {
                    '>' => OpCode::GREATER,
                    '<' => OpCode::LESS,
                    _ => OpCode::NONE,
                };
                let value: usize = i[2..colon_pos.unwrap()].parse().unwrap();
                let mut wf_id = i[colon_pos.unwrap()+1..].to_owned();
                let mut outcome = Outcome::JUMP;
                if wf_id == "A" {
                    wf_id = String::new();
                    outcome = Outcome::ACCEPTED
                } else if wf_id == "R" {
                    wf_id = String::new();
                    outcome = Outcome::REJECTED                    
                }
                instr.push(Instruction{part_id: Some(part_id), oper: op, value, wf_id, outcome});
            };
            wfs.push(Workflow{id, instructions: instr});
        }        
    }
    (wfs,parts)
}

fn get_rating(p: &Part, wfs: &Vec<Workflow>) -> usize {
    let mut wf_id = "in";
    let mut outcome = Outcome::JUMP;
    while outcome == Outcome::JUMP {
        let wf = wfs.iter().find(|&w| *w.id == *wf_id ).unwrap();
        for i in &wf.instructions {
            if i.oper == OpCode::NONE 
                || i.oper == OpCode::GREATER && p.values[i.part_id.unwrap()] > i.value 
                || i.oper == OpCode::LESS && p.values[i.part_id.unwrap()] < i.value {
                outcome = i.outcome;
                wf_id = &i.wf_id;
                break;
            }               
        }
    }
    if outcome == Outcome::REJECTED { return 0;}
    p.values.iter().sum()
}

fn part_1(filename: &str) {   
    let (wfs,parts) = parse(filename);
    let mut ans = 0;
    for p in parts{
        ans += get_rating(&p, &wfs);
    }
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    let ans = 0;
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_sample.txt");
}
