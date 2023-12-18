use std::fs;
use std::cmp;

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

fn trace_beam(c: &Vec<Vec<char>>, b: &Beam, traced: &mut Vec<Beam>) -> Vec<Beam> {
    let mut beams: Vec<Beam> = vec![];
    let mut tb: Beam = Beam{xs: b.xs, ys: b.ys, xe: b.xs, ye: b.ys, dx: b.dx, dy: b.dy};
    
    if traced.iter().any(|x| x.xs == b.xs && x.ys == b.ys && x.dx == b.dx && x.dy == b.dy ){
        return beams;
    }

    //println!("Beam started [{},{}]=>[{},{}]({},{})", tb.xs+1, tb.ys+1, tb.xe+1,tb.ye+1,tb.dx,tb.dy);

    if tb.dx != 0 {        
        loop {
            let ys = tb.ys as usize;
            let xe = tb.xe as usize;
            if ( c[ys][xe] == '\\' || c[ys][xe] == '|' )
                && tb.ys + tb.dx >= 0 && tb.ys + tb.dx < c.len() as i32 {
                beams.push(Beam{xs: tb.xe, ys: tb.ys + tb.dx, 
                                xe: tb.xe, ye: tb.ys + tb.dx,
                                dx: 0, dy: tb.dx });
            }
            if ( c[ys][xe] == '/' || c[ys][xe] == '|' )
                && tb.ys - tb.dx >= 0 && tb.ys - tb.dx < c.len() as i32 {
                beams.push(Beam{xs: tb.xe, ys: tb.ys - tb.dx, 
                                xe: tb.xe, ye: tb.ys - tb.dx,
                                dx: 0, dy: -tb.dx });
            }    
            if c[ys][xe] != '.' && c[ys][xe] != '-' {
                break;
            }       
            if tb.xe + tb.dx < 0 || tb.xe + tb.dx >= c[0].len() as i32 {
                break;
            }   
            tb.xe = tb.xe + tb.dx;
        }
    }

    if tb.dy != 0 {        
        loop {
            let ye = tb.ye as usize;
            let xs = tb.xs as usize;
            if ( c[ye][xs] == '\\' || c[ye][xs] == '-'  )
                && tb.xs + tb.dy >= 0 && tb.xs + tb.dy < c[0].len() as i32 {
                    
                beams.push(Beam{xs: tb.xs + tb.dy, ys: tb.ye, 
                                xe: tb.xs + tb.dy, ye: tb.ye,
                                dx: tb.dy, dy: 0 });
            }
            if ( c[ye][xs] == '/' || c[ye][xs] == '-' )
                && tb.xs - tb.dy >= 0 && tb.xs - tb.dy < c[0].len() as i32 {

                beams.push(Beam{xs: tb.xs - tb.dy, ys: tb.ye, 
                                xe: tb.xs - tb.dy, ye: tb.ye,
                                dx: -tb.dy, dy: 0 });
            }    
            if c[ye][xs] != '.' && c[ye][xs] != '|' {
                break;
            }       
            if tb.ye + tb.dy < 0 || tb.ye + tb.dy >= c.len() as i32 {
                break;
            }   
            tb.ye = tb.ye + tb.dy;
        }
    }

    //println!("Beam ended [{},{}]=>[{},{}]({},{})", tb.xs+1, tb.ys+1, tb.xe+1,tb.ye+1,tb.dx, tb.dy);
    traced.push(tb);
    beams
}

fn calc_energized(contraption: &Vec<Vec<char>>, start_beam: Beam) -> u32 {
    let mut traced_beams : Vec<Beam> = vec![];
    let mut active_beams : Vec<Beam> = vec![];

    active_beams.push(start_beam);
    while !active_beams.is_empty(){
        let mut new_beams: Vec<Beam> = vec![];
        for b in &active_beams  {
            for nb in trace_beam(&contraption, &b, &mut traced_beams){
                new_beams.push(nb);
            }
        }
        active_beams.clear();
        active_beams.append(&mut new_beams);
    }

    // for tb in &traced_beams{
    //      println!("Beam [{},{}]=>[{},{}]({},{})", tb.xs+1, tb.ys+1, tb.xe+1,tb.ye+1,tb.dx,tb.dy);
    // }

    let mut energized = 0;
    for y in 0..contraption.len() {
        // let mut ln: String = String::new();
        for x in 0..contraption[0].len() {
            if traced_beams.iter().any(|b| b.xs <= x as i32 && b.xe >= x as i32 && b.ys <= y as i32 && b.ye >= y as i32 
                                        || b.xe <= x as i32 && b.xs >= x as i32 && b.ye <= y as i32 && b.ys >= y as i32 ) {
                // ln.push('#');
                energized += 1;
            } else {
                // ln.push('.');
            }
        }
        // println!("{ln}");
    }
    energized
}

fn part_1(filename: &str) {   
    let contraption = parse(filename);
    println!("Answer for part 1: {}",calc_energized(&contraption,Beam{xs:0, ys:0, xe:0, ye:0, dx: 1, dy:0}));
}

fn part_2(filename: &str) {
    let contraption = parse(filename);
    let mut max_energy: u32 = 0;
    for y in 0..contraption.len() {
        let mut v = calc_energized(&contraption,Beam{xs:0, ys:y as i32, xe:0, ye:y as i32, dx: 1, dy:0});
        max_energy = cmp::max(v, max_energy);
        v = calc_energized(&contraption,Beam{xs: (contraption[0].len()-1 ) as i32, ys:y as i32, xe:(contraption[0].len()-1 ) as i32, ye:y as i32, dx: -1, dy:0});
        max_energy = cmp::max(v, max_energy);
    }
    for x in 0..contraption[0].len() {
        let mut v = calc_energized(&contraption,Beam{xs:x as i32, ys:0, xe:x as i32, ye:0, dx: 0, dy:1});
        max_energy = cmp::max(v, max_energy);
        v = calc_energized(&contraption,Beam{xs: x as i32, ys: (contraption.len()-1 ) as i32, xe: x as i32, ye:(contraption.len()-1 ) as i32, dx: 0, dy: -1});
        max_energy = cmp::max(v, max_energy);
    }    
    println!("Answer for part 2: {}", max_energy);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
