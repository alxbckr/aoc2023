use std::fs;

struct Instruction{
    dir : char,
    steps : isize,
    steps2: isize,
    dir2: char,
}

impl Instruction{
    fn new(dir: char, steps: isize, steps2: isize, dir2: char) -> Self {
        Self { dir, steps, steps2, dir2 }
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
        let dir2 = match &parts[2][parts[2].len()-2..parts[2].len()-1]{
            "0" => 'R',
            "1" => 'D',
            "2" => 'L',
            "3" => 'U',
            _ => '_'
        };
        let steps2 = isize::from_str_radix(&parts[2][2..parts[2].len()-2],16).unwrap();
        res.push(Instruction::new(parts[0].chars().last().unwrap(), 
                                    parts[1].parse().unwrap(), 
                                    steps2, dir2));
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

fn get_points(instr: &Vec<Instruction>) -> Vec<Point> {
    let mut res = Vec::new();
    let mut p = Point::new(0, 0);
    for i in instr {
        match i.dir2 {
            'R' => { p.x += i.steps2; }
            'L' => { p.x -= i.steps2; }
            'U' => { p.y -= i.steps2; }
            'D' => {p.y += i.steps2; }
            _ => ()            
        }
        res.push(p);
    }      
    res
}

fn shoelace(instr: &Vec<Instruction>) -> usize {
    let points = get_points(instr);
    let mut area : isize = 0;
    let mut perimeter : isize = 0;
    for (x,p) in points.iter().enumerate(){
        if x == points.len()-1 { continue;}
        area += p.x*points[x+1].y - p.y*points[x+1].x;
        perimeter += ( p.x - points[x+1].x ).abs() + ( p.y - points[x+1].y ).abs();
    }
    area += points[points.len()-1].x*points[0].y - points[0].x*points[points.len()-1].y;
    perimeter += (points[points.len()-1].x - points[0].x).abs() + (points[points.len()-1].y - points[0].y).abs();
    ( ( ( area + perimeter ) / 2) ) as usize + 1
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
    let instr = parse(filename);
    let ans = shoelace(&instr);
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
