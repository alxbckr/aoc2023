use std::fs;

struct Dir {
    dx : i32,
    dy : i32,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<char>> {
    let mut res: Vec<Vec<char>> = vec![];
    for line in read_file(filename) {
        res.push(line.chars().collect());
    }
    res
}

fn move_rocks(board: &mut Vec<Vec<char>>, dir: &Dir) {
    let mut rocks_moved = true;
    while rocks_moved {       
        rocks_moved = false;
        let mut yc:Vec<usize> = (0..board.len()).collect();
        let mut xc:Vec<usize> = (0..board[0].len()).collect();
        if dir.dy == -1 {
            yc = (1..board.len()).collect();
        } else if dir.dy == 1 {
            yc = (0..board.len()-1).collect();
            yc.reverse();
        } else if dir.dx == -1 {
            xc = (1..board[0].len()).collect();
        } else if dir.dx == 1 {
            xc = (0..board[0].len()-1).collect();
            xc.reverse();
        }
        for y in yc {
            for x in &xc {
                if  board[y][*x] == 'O' && board[((y as i32)+dir.dy) as usize][((*x as i32)+dir.dx) as usize] == '.' {
                    rocks_moved = true;
                    board[((y as i32)+dir.dy) as usize][((*x as i32)+dir.dx) as usize] = 'O';
                    board[y][*x] = '.';
                }
            }
        }
    }
}

fn print_board(board: &Vec<Vec<char>>){
    for b in board {
        let s: String = b.iter().collect();
        println!("{s}");
    }
    println!("");
}

fn part_1(filename: &str) {   
    let mut board = parse(filename);
    move_rocks(&mut board, &Dir{dx: 0, dy: -1});
    let ans = 
        board.iter()
        .enumerate().
        fold(0, |s,(y,b)| 
            s + b.iter().filter(|&x| *x == 'O').count() * (board.len()-y) );
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    let mut board = parse(filename);
    for i in 0..1000 {
        move_rocks(&mut board, &Dir{dx: 0, dy: -1});
        move_rocks(&mut board, &Dir{dx: -1, dy: 0});
        move_rocks(&mut board, &Dir{dx: 0, dy: 1});
        move_rocks(&mut board, &Dir{dx: 1, dy: 0});        
    }
    let ans = 
        board.iter()
        .enumerate().
        fold(0, |s,(y,b)| 
            s + b.iter().filter(|&x| *x == 'O').count() * (board.len()-y) );
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
