use std::fs;

struct Lens{
    id: String,
    value: u32,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    for line in read_file(filename) {
        let mut spl = line.split(",").map(|s| s.to_string()).collect();
        res.append(&mut spl);
    }
    res
}

fn calc_hash(s: &String) -> u32 {
    let mut val = 0;
    for c in s.chars(){
        val = ( val + ( c as u32 ) ) * 17 % 256;
    }
    val
}

fn part_1(filename: &str) {   
    let mut hash = 0;
    for s in parse(filename){
        //println!("s={s} hash={}",calc_hash(&s));
        hash += calc_hash(&s);
    }
    println!("Answer for part 1: {}",hash);
}

fn part_2(filename: &str) {
    let mut boxes: Vec<Vec<Lens>> = vec![];
    for _ in 0..256{
        boxes.push(Vec::new()); 
    }
    for s in parse(filename){
        let box_id: usize;
        if s.chars().last().unwrap() == '-' {
             let id = s[0..s.len()-1].to_string();
             box_id = calc_hash(&id) as usize;
             if let Some(i) = boxes[box_id].iter().position(|x| x.id == id) {
                boxes[box_id].remove(i);
             }
        } else {
            let p : Vec<_> = s.split("=").collect();
            box_id = calc_hash(&p[0].to_string()) as usize;
            if let Some(i) = &boxes[box_id].iter().position(|x| x.id == p[0]) {
                boxes[box_id][*i].value = p[1].parse().unwrap();
            } else {
                boxes[box_id].push(Lens{ id: p[0].to_string(), value: p[1].parse().unwrap()});
            }
        }
    }    

    let mut sum:u32 = 0;
    for (i,b) in boxes.iter().enumerate(){
        for (j,e) in b.iter().enumerate() {
            sum += (i as u32 +1) * (j as u32 +1) * e.value as u32;
            //print!("[{} {}]", e.id, e.value);
        }
        //println!("");
    }
    //println!("");

    println!("Answer for part 2: {}",sum);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
