use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    x: usize,
    y: usize,
}

// The priority queue depends on `Ord`.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
            .then_with(|| self.x.cmp(&other.x))
            .then_with(|| self.y.cmp(&other.y))
    }
}

fn step(map: &Vec<Vec<usize>>, heap: &mut BinaryHeap<State>, 
            dist: &mut Vec<Vec<usize>>, prev: &mut Vec<Vec<(usize,usize)>>,
            cost: usize, x: usize, y : usize, p: (usize, usize)) {
    let next = State{ cost: cost + map[y][x], x, y };
    if next.cost < dist[y][x] {
        heap.push(next);
        dist[y][x] = next.cost;
        prev[y][x] = p;
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

fn check_prev(prev: &Vec<Vec<(usize,usize)>>, xc: usize, yc:usize) -> bool {
    let mut x = xc;
    let mut y = yc;
    let mut x_ok : bool = false;
    let mut y_ok : bool = false;
    for _ in 0..4 {
        x = prev[y][x].0;
        y = prev[y][x].1;
        if x == 0 && y == 0 {
            return true;
        }
        if x != xc {            
            x_ok = true;
        }
        if y != yc {
            y_ok = true;
        }
    }
    x_ok && y_ok
}

fn dijkstra(map: &Vec<Vec<usize>>, xs: usize, ys: usize, xe: usize, ye: usize) -> usize {
    // distance from start to node
    let mut dist: Vec<Vec<usize>> = vec![];
    let mut prev: Vec<Vec<(usize,usize)>> = vec![];
    for _ in 0..map.len() {
        dist.push((0..map[0].len()).map(|_| usize::MAX).collect());
        prev.push((0..map[0].len()).map(|_| (0,0)).collect());
    }

    let mut heap: BinaryHeap<State> = BinaryHeap::new();
    dist[ys][xs] = map[ys][xs];
    heap.push(State{ cost: map[ys][xs], x: xs, y: ys});

    while let Some(State { cost, x, y }) = heap.pop() {
        // found
        if x == xe && y == ye { 
            draw_path(map, &prev, xs, ys, xe, ye);
            return cost; }
        // Important as we may have already found a better way
        if cost > dist[y][x] || !check_prev(&prev, x, y) { continue; } 

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        if y >= 1 {
            step(&map, &mut heap, &mut dist, &mut prev, cost, x, y-1, (x,y));
        }
        if y < map.len()-1 {
            step(&map, &mut heap, &mut dist, &mut prev, cost, x, y+1, (x,y));
        }        
        if x >= 1 {
            step(&map, &mut heap, &mut dist, &mut prev, cost, x-1, y, (x,y));
        }
        if x < map[0].len()-1 {
            step(&map, &mut heap, &mut dist, &mut prev, cost, x+1, y, (x,y));
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
