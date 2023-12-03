use std::fs;

struct Number{
    num: String,
    left: i32,
    right: i32
}

impl Number {
    pub fn new() -> Self {
        Number {
            num: String::new(),
            left: 0,
            right: 0,
        }
    }
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn check_num(n: &Number, y: i32, symbols: &Vec<Vec<i32>>) -> bool {
    if y < 0 || y >= symbols.len() as i32 { return false; };
    for s in &symbols[y as usize] {
        if *s <= n.right + 1 && *s >= n.left - 1 { return  true;};
    }
    return  false;
}

fn find_adj_num(ys: i32, xs: i32, numbers: &Vec<Vec<Number>>) -> Vec<u32> {
    let mut res: Vec<u32> = vec![];
    for y in ys-1..=ys+1 {
        if y < 0 || y >= numbers.len() as i32 { continue; };
        for n in &*numbers[y as usize] {
            if xs >= n.left - 1 && xs <= n.right + 1 {
                res.push(n.num.parse().unwrap());
            }
        }
    }
    return  res;
}

fn part_1(filename: &str){
    let mut numbers: Vec<Vec<Number>> = vec![];
    let mut symbols: Vec<Vec<i32>> = vec![];
    let lines = read_file(filename);

    for line in lines {
        let mut number_line:Vec<Number> = vec![];
        let mut symbol_line:Vec<i32> = vec![];
        let mut num = Number::new();
        let mut is_number: bool = false;
        for (x,c) in line.char_indices() {
            if c.is_numeric() {
                if !is_number { 
                    num.num.clear();
                    num.left = x as i32;
                };
                is_number = true;
                num.num.push(c);
            } else if c != '.' {
                symbol_line.push(x as i32);
            }           
            if (!c.is_numeric() || x == line.len()-1 ) && is_number {
                if !c.is_numeric() {
                    num.right = x as i32 -1;
                } else {
                    num.right = x as i32;
                }
                number_line.push(num);     
                num = Number::new();      
                is_number = false;     
            }  
        }
        numbers.push(number_line);
        symbols.push(symbol_line);
    }

    let mut sum = 0;
    for (y, num_line) in numbers.iter().enumerate() {
        for n in num_line {
            if check_num(&n, y as i32, &symbols) || 
               check_num(&n, y as i32 - 1, &symbols) ||
               check_num(&n, y as i32 + 1, &symbols) {
                sum += n.num.parse::<u32>().unwrap();
            }
        }
    }
    
    println!("Answer for part 1 is {}", sum);
}

fn part_2(filename: &str){
    let mut numbers: Vec<Vec<Number>> = vec![];
    let mut symbols: Vec<Vec<i32>> = vec![];
    let lines = read_file(filename);

    for line in lines {
        let mut number_line:Vec<Number> = vec![];
        let mut symbol_line:Vec<i32> = vec![];
        let mut num = Number::new();
        let mut is_number: bool = false;
        for (x,c) in line.char_indices() {
            if c.is_numeric() {
                if !is_number { 
                    num.num.clear();
                    num.left = x as i32;
                };
                is_number = true;
                num.num.push(c);
            } else if c == '*' {
                symbol_line.push(x as i32);
            }           
            if (!c.is_numeric() || x == line.len()-1 ) && is_number {
                if !c.is_numeric() {
                    num.right = x as i32 -1;
                } else {
                    num.right = x as i32;
                }
                number_line.push(num);     
                num = Number::new();      
                is_number = false;     
            }  
        }
        numbers.push(number_line);
        symbols.push(symbol_line);
    }

    let mut sum = 0;
    for (y, sym_line) in symbols.iter().enumerate() {
        for x in sym_line {
            let adj = find_adj_num(y as i32, *x, &numbers);
            if adj.len() == 2 {
                sum += adj[0] * adj[1];
            }
        }
    }
    
    println!("Answer for part 2 is {}", sum);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
