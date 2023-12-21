use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
    dir: Direction,
    dir_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point{
    x: usize,
    y: usize,
    dir: Direction
}

impl Point {
    fn new(x: usize, y: usize, dir: Direction) -> Self {
        Self { x, y, dir }
    }
}

// The priority queue depends on `Ord`.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}
fn draw_path(map: &Vec<Vec<usize>>, prev: &Vec<Vec<(usize,usize)>>,
                xs: usize, ys: usize, xe: usize, ye: usize){
    let mut x = xe;
    let mut y = ye;
    let mut path: Vec<(usize,usize)> = vec![];
    while x != xs || y != ys {
        path.push((x,y));
        (x,y) = prev[y][x];
    }

    for y in 0..map.len(){
        let mut s: String = String::new();
        for x in 0..map[0].len(){
            if path.iter().any(|(xp,yp)| *xp == x && *yp == y) {
                s.push('#');
            } else {
                s.push(char::from_digit(map[y][x] as u32,10).unwrap());
            }
        }
        println!("{s}");
    }
}

fn next_points(map: &Vec<Vec<usize>>, x: usize, y: usize) -> Vec<Point> {
    let mut next = Vec::new();
    if y > 0 {
        next.push(Point::new(x, y-1, Direction::North));
    }
    if x > 0 {
        next.push(Point::new(x-1, y, Direction::West));
    }    
    if y < map.len()-1 {
        next.push(Point::new(x, y+1, Direction::South));
    }
    if x < map[0].len()-1 {
        next.push(Point::new(x+1, y, Direction::East));
    }
    next
}

fn dijkstra(map: &Vec<Vec<usize>>, xs: usize, ys: usize, xe: usize, ye: usize) -> usize {
    // distance from start to node
    let mut prev: Vec<Vec<(usize,usize)>> = vec![];
    for _ in 0..map.len() {
        prev.push((0..map[0].len()).map(|_| (0,0)).collect());
    }

    let mut distances = HashMap::new();
    distances.insert(Point::new(xs, ys, Direction::South), 0);
    distances.insert(Point::new(xs, ys, Direction::East), 0);

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    heap.push(State{ cost: map[ys][xs], x: xs, y: ys, dir: Direction::East, dir_count: 0});
    heap.push(State{ cost: map[ys][xs], x: xs, y: ys, dir: Direction::North, dir_count: 0});

    while let Some(State { cost, x, y, dir, dir_count }) = heap.pop() {
        // found
        if x == xe && y == ye { 
            // draw_path(map, &prev, xs, ys, xe, ye);
            return cost; 
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for p in next_points(map, x, y) {
            
            let new_cost = cost + map[p.y][p.x];
            if let Some(&best) = distances.get(&p) {
                if new_cost >= best {
                    continue;
                }
            }

            let mut new_dir_count = dir_count;
            if p.dir == dir {
                new_dir_count += 1;
            } else {
                new_dir_count = 0;
            }
            if new_dir_count > 3 { continue;}

            let next = State{ cost: new_cost, x: p.x, y: p.y, dir: p.dir, dir_count: new_dir_count };
            
            heap.push(next);
            distances.insert(p.clone(),new_cost);
            prev[p.y][p.x] = (x,y);
        }
    }   
    0
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<usize>> {
    let mut res: Vec<Vec<usize>> = vec![];
    for line in read_file(filename) {
        res.push(line.chars().map(|y| y.to_digit(10).unwrap() as usize).collect());
    }
    res
}


fn part_1(filename: &str) {   
    let map = parse(filename);
    let ans = dijkstra(&map, 0, 0, map[0].len()-1, map.len()-1);
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    println!("Answer for part 2: {}", 0);
}

fn main() {
    part_1("puzzle_sample.txt");
    part_2("puzzle_sample.txt");
}
