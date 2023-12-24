use std::fs;

struct Instruction{
    dir : char,
    steps : isize,
    color: String,
}

impl Instruction{
    fn new(dir: char, steps: isize, color: String) -> Self {
        Self { dir, steps, color }
    }
}
#[derive(Debug,Copy, Clone)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Instruction> {
    let mut res = Vec::new();
    for line in read_file(filename) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        res.push(Instruction::new(parts[0].chars().last().unwrap(), 
                                    parts[1].parse().unwrap(), 
                                    parts[2][1..parts[2].len()-1].to_string()))
    }   
    res
}

fn get_boundaries(instr: &Vec<Instruction>) -> (Point, Point) {
    let mut p = Point::new(0, 0);
    let mut max = Point::new(0, 0);
    let mut min = Point::new(0, 0);
    let mut s = Point::new(0, 0);
    for i in instr {
        match i.dir {
            'R' => { p.x += i.steps; }
            'L' => { p.x -= i.steps; }
            'U' => { p.y -= i.steps; }
            'D' => {p.y += i.steps; }
            _ => ()            
        }
        if p.x > max.x { max.x = p.x };
        if p.y > max.y { max.y = p.y };
        if p.x < min.x { min.x = p.x };
        if p.y < min.y { min.y = p.y };
    }    
    p.x = max.x - min.x + 1;
    p.y = max.y - min.y + 1;
    s.x = s.x - min.x;
    s.y = s.y - min.y;
    (p,s)    
}

fn dig_tunnel(cave: &mut Vec<Vec<char>>, p: &Point, steps: isize, dx: isize, dy: isize) {
   if dx != 0 {
        let mut x = p.x;
        for _ in 0..steps {
            x += dx;
            cave[p.y as usize][x as usize] = '#';
        }
    }
    if dy != 0 {
        let mut y = p.y;
        for _ in 0..steps {
            y += dy;
            cave[y as usize][p.x as usize] = '#';
        }
    }    
}

fn dig_cave(instr: &Vec<Instruction>) -> (Vec<Vec<char>>,Point) {
    let (max,start) = get_boundaries(instr);
    let mut cave: Vec<Vec<char>> = vec![vec!['.';max.x as usize];max.y as usize];
    let mut p = start; 
    cave[p.y as usize][p.x as usize] = '#';
    for i in instr {
        match i.dir {
            'R' => { 
                dig_tunnel(&mut cave, &p, i.steps, 1, 0); 
                p.x += i.steps;
            }
            'L' => { 
                dig_tunnel(&mut cave, &p, i.steps, -1, 0); 
                p.x -= i.steps; 
            }
            'U' => { 
                dig_tunnel(&mut cave, &p, i.steps, 0, -1); 
                p.y -= i.steps; 
            }
            'D' => {
                dig_tunnel(&mut cave, &p, i.steps, 0, 1); 
                p.y += i.steps; 
            }
            _ => ()            
        }        
        cave[p.y as usize][p.x as usize] = '#';
    }
    (cave,start)
}

fn fill_cave(cave: &mut Vec<Vec<char>>, start: &Point ){
    let mut stack = Vec::new();
    let mut p = *start;
    stack.push(p);
    while !stack.is_empty(){
        p = stack.pop().unwrap();        
        cave[p.y as usize][p.x as usize] = '#';
        if p.x > 0 && cave[p.y as usize][(p.x - 1) as usize] == '.' { stack.push(Point::new(p.x-1, p.y));}
        if p.x < cave[0].len() as isize && cave[p.y as usize][(p.x + 1) as usize] == '.' { stack.push(Point::new(p.x+1, p.y));}
        if p.y > 0 && cave[(p.y-1) as usize][p.x as usize] == '.' { stack.push(Point::new(p.x, p.y-1));}
        if p.y < cave.len() as isize && cave[(p.y+1) as usize][p.x as usize] == '.' { stack.push(Point::new(p.x, p.y+1));}        
    }
}

fn part_1(filename: &str) {   
    let instr = parse(filename);
    let (mut cave,start) = dig_cave(&instr);
    fill_cave(&mut cave, &Point::new(start.x+1, start.y+1));
    for l in &cave {
        for c in l {
            print!("{}", c);
        }
        println!();
    }
    let ans = cave.iter().flat_map(|x| x.iter().filter(|&c| *c == '#')).count();
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    let map = parse(filename);
    let ans = 0;
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_sample.txt");
}
