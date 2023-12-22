use std::fs;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    // Return the opposite direction.
    fn opposite(&self) -> Self {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl  Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn valid_next(&self, grid: &[Vec<usize>]) -> Vec<(Direction, Point)> {
        let mut next = Vec::new();
        if self.x > 0 {
            next.push((Direction::West, Self::new(self.x - 1, self.y)));
        }
        if self.y > 0 {
            next.push((Direction::North, Self::new(self.x, self.y - 1)));
        }
        if self.x < grid[0].len() - 1 {
            next.push((Direction::East, Self::new(self.x + 1, self.y)));
        }
        if self.y < grid.len() - 1 {
            next.push((Direction::South, Self::new(self.x, self.y + 1)));
        }
        next
    }      
}    


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    node: Node,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Node{
    pos: Point,
    dir: Direction,
    dir_count: usize,
}

impl Node {
    fn new(pos: Point, dir: Direction, dir_count: usize) -> Self {
        Self { pos, dir, dir_count }
    }
}

// The priority queue depends on `Ord`.
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // We are using a min heap, so we are doing this backwards.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn draw_path(map: &Vec<Vec<usize>>, prev: &HashMap<Node,Node>,
                start: &Point, end: &Point){
    let mut path: Vec<Point> = vec![];
    let mut p = prev.iter().find(|(k,_)| k.pos == *end ).unwrap().0;
    while p.pos != *start {
        path.push(p.pos.clone());
        p = prev.get(p).unwrap();
    }

    for y in 0..map.len(){
        let mut s: String = String::new();
        for x in 0..map[0].len(){
            if path.iter().any(|p| p.x == x && p.y == y) {
                s.push('#');
            } else {
                s.push(char::from_digit(map[y][x] as u32,10).unwrap());
            }
        }
        println!("{s}");
    }
}

fn next_nodes(map: &Vec<Vec<usize>>, node: &Node) -> Vec<Node> {
    let mut next = Vec::new();
    for (d,p) in node.pos.valid_next(map) {
        if d == node.dir.opposite() { continue; }
        if d != node.dir {
            next.push(Node::new(p, d,1));
        } else if node.dir_count < 3 {
            next.push(Node::new(p, d, node.dir_count + 1));
        }
    }
    next
}

fn next_nodes2(map: &Vec<Vec<usize>>, node: &Node) -> Vec<Node> {
    let mut next = Vec::new();
    for (d,p) in node.pos.valid_next(map) {
        if d == node.dir.opposite() { continue; }
        if d != node.dir && node.dir_count >= 4 {
            next.push(Node::new(p, d, 1));
        } else if d == node.dir && node.dir_count < 10 {
            next.push(Node::new(p, d, node.dir_count + 1));
        }
    }
    next
}

fn dijkstra(map: &Vec<Vec<usize>>, start: &Point, end: &Point) -> usize {
    // distance from start to node
    let mut prev = HashMap::new();
    let mut distances = HashMap::new();
    distances.insert(Node::new(start.clone(),Direction::South,0), 0);
    distances.insert(Node::new(start.clone(), Direction::East,0), 0);

    let mut heap = BinaryHeap::new();
    heap.push(State{ cost: 0, node: Node::new(start.clone(), Direction::South,0)});
    heap.push(State{ cost: 0, node: Node::new(start.clone(), Direction::East,0)});

    while let Some(State { cost, node}) = heap.pop() {
        // found
        if node.pos == *end { 
            draw_path(map, &prev, start, end);
            return cost; 
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbor in next_nodes(map, &node) {           
            let new_cost = cost + map[neighbor.pos.y][neighbor.pos.x];
            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            } 
            distances.insert(neighbor.clone(),new_cost);
            heap.push(State{ cost: new_cost, node: neighbor.clone() });
            prev.insert(neighbor.clone(), node.clone());
        }
    }   
    0
}

fn dijkstra2(map: &Vec<Vec<usize>>, start: &Point, end: &Point) -> usize {
    // distance from start to node
    let mut prev = HashMap::new();
    let mut distances = HashMap::new();
    distances.insert(Node::new(start.clone(),Direction::South,0), 0);
    distances.insert(Node::new(start.clone(), Direction::East,0), 0);

    let mut heap = BinaryHeap::new();
    heap.push(State{ cost: 0, node: Node::new(start.clone(), Direction::South,0)});
    heap.push(State{ cost: 0, node: Node::new(start.clone(), Direction::East,0)});

    while let Some(State { cost, node}) = heap.pop() {
        // found
        if node.pos == *end { 
            draw_path(map, &prev, start, end);
            return cost; 
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for neighbor in next_nodes2(map, &node) {           
            let new_cost = cost + map[neighbor.pos.y][neighbor.pos.x];
            if let Some(&best) = distances.get(&neighbor) {
                if new_cost >= best {
                    continue;
                }
            } 
            distances.insert(neighbor.clone(),new_cost);
            heap.push(State{ cost: new_cost, node: neighbor.clone() });
            prev.insert(neighbor.clone(), node.clone());
        }
    }   
    0
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
    let ans = dijkstra(&map, &Point::new(0, 0), &Point::new(map[0].len()-1, map.len()-1));
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    let map = parse(filename);
    let ans = dijkstra2(&map, &Point::new(0, 0), &Point::new(map[0].len()-1, map.len()-1));
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
