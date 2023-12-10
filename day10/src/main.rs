use std::fs;
use inline_colorization::*;

struct Coord{
    x: usize,
    y: usize,
}

struct Dir{
    dx: i64,
    dy: i64,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<char>> {
    let mut res = vec![];
    read_file(filename).iter().for_each(|x| res.push(x.chars().collect()));
    res
}

fn find_start(inp: &Vec<Vec<char>>) -> Coord {
    let mut sx = 0;
    let mut sy = 0;
    for (y,e) in inp.iter().enumerate( ){
        sy = y;
        if let Some(px) = e.iter().position(|x| *x == 'S') { 
            sx = px;
            break;
        }
    }    
    Coord{ x: sx, y: sy }
}

fn step(s: &Coord, dir: &Dir, field: &Vec<Vec<char>>, max: &Coord) -> Option<(Coord, Dir)> {
    let nx = s.x as i64 + dir.dx;
    let ny = s.y as i64 + dir.dy;
    if nx < 0 || nx > max.x as i64 || ny < 0 || ny > max.y as i64 {         
        return  None;
    };

    let n = Coord {x: nx as usize, y: ny as usize};
    let mut new_dir = Dir{dx: 0, dy:0};
    let c = field[n.y][n.x];
    if c == 'S' {
        return Some((n, new_dir));
    }

    if ( c == '-' && dir.dx == 1 ) || ( c == 'F' && dir.dy == -1 ) || ( c == 'L' && dir.dy == 1 ) {
        new_dir.dx = 1;
    } else if ( c == '-' && dir.dx == -1 ) || ( c == 'J' && dir.dy == 1 ) || ( c == '7'  && dir.dy == -1 ) {
        new_dir.dx = -1;
    } else if ( c == '|' && dir.dy == -1 ) || ( c == 'L' && dir.dx == -1 ) || ( c == 'J'  && dir.dx == 1 ) {
        new_dir.dy = -1;
    } else if ( c == '|' && dir.dy == 1 ) || ( c == 'F' && dir.dx == -1 ) || ( c == '7' && dir.dx == 1 ) {
        new_dir.dy = 1;
    } else {
        return None;
    }
    Some((n, new_dir))
}

fn build_cycle(start: &Coord, start_dir: &Dir, field: &Vec<Vec<char>>) -> Vec<Coord> {
    let mut path: Vec<Coord> = vec![];
    let mut curr = Coord { x: start.x, y: start.y };
    let mut dir = Dir{dx: start_dir.dx, dy: start_dir.dy};
    let max = &Coord { x: field[0].len()-1, y: field.len()-1 };
    loop {
        if let Some((new_coord,new_dir)) = step(&curr, &dir, &field, &max) {
            curr = new_coord;
            dir = new_dir;
            //println!("Curr {}  {} {}", field[curr.y][curr.x], curr.y, curr.x);
            //println!("Dir {} {}", dir.dy, dir.dx);
            path.push(Coord { x: curr.x, y: curr.y });
            if field[curr.y][curr.x] == 'S' {
                return path;
            }
            continue;
        } else {
            return path;
        }
    }    
}

fn print_path(path: &Vec<Coord>, field: &Vec<Vec<char>>, el_in: &Vec<Coord>) {
    for (y, line) in field.iter().enumerate() {
        for (x,c) in line.iter().enumerate() {
            if *c == 'S' {
                print!("{color_green}{c}{color_reset}");
            } else if path.iter().any(|e| e.x == x && e.y == y) {
                let mc = match c {
                    '7' => '┐',
                    'L' => '└',
                    'J' => '┘',
                    'F' => '┌',
                    _ => *c
                };
                print!("{color_red}{mc}{color_reset}");
            } else if el_in.iter().any(|e| e.x == x && e.y == y) {
                print!("{color_green}*{color_reset}");
            }
             else {
                print!("{}",c);
            }
        }
        print!("\n");
    }
}

fn part_1(filename: &str) {
    let field = parse(filename);
    let s = find_start(&field);
    let mut path = build_cycle(&s, &Dir{dx: 1, dy:0}, &field);    
    if path.is_empty() {
        path = build_cycle(&s, &Dir{dx: 0, dy:1}, &field);
    }    
    if path.is_empty() {
        path = build_cycle(&s, &Dir{dx: -1, dy:0}, &field);
    }
    if path.is_empty() {
        path = build_cycle(&s, &Dir{dx: 0, dy:-1}, &field);
    }    
    let el_in: Vec<Coord> = vec![];
    print_path(&path, &field, &el_in);

    println!("start at {} {}", s.x, s.y);
    println!("Answer for part 1: {}",path.len() / 2);
}

fn part_2(filename: &str) {
    let field = parse(filename);
    let s = find_start(&field);
    let mut path = build_cycle(&s, &Dir{dx: 1, dy:0}, &field);    
    if path.is_empty() {
        path = build_cycle(&s, &Dir{dx: 0, dy:1}, &field);
    }    
    if path.is_empty() {
        path = build_cycle(&s, &Dir{dx: -1, dy:0}, &field);
    }
    if path.is_empty() {
        path = build_cycle(&s, &Dir{dx: 0, dy:-1}, &field);
    }    
    
    let mut el_in: Vec<Coord> = vec![];
    for (y,line) in field.iter().enumerate() {        
        let mut pl: Vec<Coord> = path.iter().filter(|p| p.y == y).map(|e| Coord{x: e.x, y: e.y}).collect();
        pl.sort_by(|a,b| a.x.partial_cmp(&b.x).unwrap());
        let mut cnt: f32 = 0.0;
        for (x,c) in line.iter().enumerate() {
            if pl.iter().any(|e| e.x == x && e.y == y) {
                if *c == 'S' {
                    cnt += 1.0;
                } else if *c == '|' {
                    cnt += 1.0;
                } else if *c == '7' || *c == 'L' {
                    cnt += 0.5;
                } else if *c == 'F' || *c == 'J' {
                    cnt -= 0.5;
                } 
            } else  if cnt % 2.0 != 0.0 {
                    el_in.push(Coord { x: x, y: y });
                    println!("Inside {} {}", x, y);
                }
            }
    }

    print_path(&path, &field, &el_in);
    println!("start at {} {}", s.x, s.y);
    println!("Answer for part 2: {}",el_in.len());
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
