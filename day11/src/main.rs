use std::fs;
use inline_colorization::*;

struct Coord {
    x: u64,
    y: u64,
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

fn expand(map: &Vec<Vec<char>>, galaxies: Vec<Coord>, factor: usize) -> Vec<Coord> {
    let mut res: Vec<Coord> = vec![];
    let mut exp_x: Vec<usize> = vec![];
    let mut exp_y: Vec<usize> = vec![];
    // which columns to add
    for x in 0..map[0].len() {
        if map.iter().all(|e| e[x] == '.' ) {
            exp_x.push(x);
        }
    }
    // which rows to add
    for y in 0..map.len() {
        if map[y].iter().all(|e| *e == '.' ) {
            exp_y.push(y);        
        }
    }
    for g in galaxies {
        res.push(Coord{ x: g.x + ( exp_x.iter().filter(|&e| *e < g.x as usize ).count() * (factor - 1) ) as u64,
                        y: g.y + ( exp_y.iter().filter(|&e| *e < g.y as usize ).count() * (factor - 1) ) as u64});
    }    
    res
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map {
        for c in line {
            if *c == '#' {
                print!("{color_red}{c}{color_reset}");
            } else {
                print!("{c}");
            }
        }
        print!("\n");
    }
}

fn find_galaxies(map: &Vec<Vec<char>>) -> Vec<Coord> {
    let mut res: Vec<Coord> = vec![];
    for (y,line) in map.iter().enumerate() {
        for (x,c) in line.iter().enumerate() {
            if *c == '#' {
                res.push(Coord{x: x as u64, y: y as u64});
            }
        }
    }    
    res
}

fn calc_min_dist(g1: &Coord, g2: &Coord) -> u64 {
    ( ( g1.x as i64 - g2.x as i64 ).abs() + ( g1.y as i64 - g2.y as i64 ).abs() ) as u64
}

fn part_1(filename: &str) {
    let map = parse(filename);
    let mut g = find_galaxies(&map);
    g = expand(&map, g, 2);
    let mut sum_dist = 0;
    for g1 in 0..g.len() {
        for g2 in g1+1..g.len() {
            let dist = calc_min_dist(&g[g1], &g[g2]);
            sum_dist += dist;
            //println!("Dist between {}.{} {}.{} is {}",g[g1].x, g[g1].y, g[g2].x, g[g2].y, dist);
        }
    }
    print_map(&map);
    println!("Answer for part 1: {}",sum_dist);
}

fn part_2(filename: &str) {
    let map = parse(filename);
    let mut g = find_galaxies(&map);
    g = expand(&map, g, 1000000);
    let mut sum_dist: u64 = 0;
    for g1 in 0..g.len() {
        for g2 in g1+1..g.len() {
            let dist = calc_min_dist(&g[g1], &g[g2]);
            sum_dist += dist;
            //println!("Dist between {}.{} {}.{} is {}",g[g1].x, g[g1].y, g[g2].x, g[g2].y, dist);
        }
    }
    println!("Answer for part 2: {}",sum_dist);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
