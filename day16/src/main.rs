use std::fs;

struct Beam {
    xs : i32,
    ys: i32,
    xe : i32,
    ye : i32,
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

fn trace_beam(c: &Vec<Vec<char>>, b: &Beam) -> Vec<Beam> {
    let mut beams: Vec<Beam> = vec![];
    let mut tb: Beam = Beam{xs: b.xs, ys: b.ys, xe: b.xs, ye: b.ye, dx: b.dx, dy: b.dy};
    
    if tb.dx != 0 {        
        loop {
            let ys = tb.ys as usize;
            let xe = tb.xe as usize;
            if c[ys][xe] == '\\' || c[ys][xe] == '|' 
                && tb.ys + tb.dx > 0 && tb.ys + tb.dx < c[0].len() as i32 {
                    
                beams.push(Beam{xs: tb.xe, ys: tb.ys + tb.dx, 
                                xe: tb.xe, ye: tb.ys + tb.dx,
                                dx: 0, dy: tb.dx });
            }
            if c[ys][xe] == '/' || c[ys][xe] == '|' 
                && tb.ys - tb.dx > 0 && tb.ys - tb.dx < c[0].len() as i32 {

                beams.push(Beam{xs: tb.xe, ys: tb.ys - tb.dx, 
                                xe: tb.xe, ye: tb.ys - tb.dx,
                                dx: 0, dy: -tb.dx });
            }    
            if c[ys][xe] != '.' && c[ys][xe] != '-' {
                break;
            }       
            if tb.xe + tb.dx < 0 || tb.xe + tb.dx > c[0].len() as i32 {
                break;
            }   
            tb.xe = tb.xe + tb.dx;
        }
    }

    if tb.dy != 0 {        
        loop {
            let ye = tb.ye as usize;
            let xs = tb.xs as usize;
            if c[ye][xs] == '\\' || c[ye][xs] == '|' 
                && tb.xe + tb.dy > 0 && tb.ys + tb.dx < c[0].len() as i32 {
                    
                beams.push(Beam{xs: tb.xs, ys: tb.ye + tb.dy, 
                                xe: tb.xe, ye: tb.ye + tb.dy,
                                dx: tb.dy, dy: 0 });
            }
            if c[ys][xe] == '/' || c[ys][xe] == '|' 
                && tb.ys - tb.dx > 0 && tb.ys - tb.dx < c[0].len() as i32 {

                beams.push(Beam{xs: tb.xe, ys: tb.ys - tb.dx, 
                                xe: tb.xe, ye: tb.ys - tb.dx,
                                dx: 0, dy: -tb.dx });
            }    
            if c[ys][xe] != '.' && c[ys][xe] != '-' {
                break;
            }       
            if tb.xe + tb.dx < 0 || tb.xe + tb.dx > c[0].len() as i32 {
                break;
            }   
            tb.xe = tb.xe + tb.dx;
        }
    }

    println!("Beam [{},{}]=>[{},{}]({},{})", tb.xs+1, tb.ys+1, tb.xe+1,tb.ye+1,tb.dx,tb.dy);
    beams
}

fn part_1(filename: &str) {   
    let contraption = parse(filename);
    let mut traced_beams : Vec<Beam> = vec![];
    let mut active_beams : Vec<Beam> = vec![];

    active_beams.push(Beam{xs:0, ys:0, xe:0, ye:0, dx: 1, dy:0});
    while !active_beams.is_empty(){
        let mut new_beams: Vec<Beam> = vec![];
        for b in &active_beams  {
            new_beams = trace_beam(&contraption, &b);
        }
        active_beams.clear();
        active_beams.append(&mut new_beams);
    }

    println!("Answer for part 1: {}",0);
}

fn part_2(filename: &str) {
    println!("Answer for part 2: {}",0);
}

fn main() {
    part_1("puzzle_sample.txt");
    part_2("puzzle_sample.txt");
}
