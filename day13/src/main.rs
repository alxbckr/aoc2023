use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<Vec<char>>> {
    let mut res: Vec<Vec<Vec<char>>> = vec![];
    let mut pattern: Vec<Vec<char>> = vec![];
    for line in read_file(filename) {
        if line.is_empty(){
            res.push(pattern);
            pattern = Vec::new();
            continue;
        }        
        pattern.push(line.chars().collect());
    }    
    res.push(pattern);
    res
}

fn find_hor_symmetry(p: &Vec<Vec<char>>) -> usize {
    let mut ident_lines: Vec<(usize,usize)> = vec![];
    for y1 in 0..p.len()-1 {
        if p[y1].iter().zip(&p[y1+1]).filter(|&(a,b)| *a == *b).count() == p[y1].len() {
            ident_lines.push((y1,y1+1));
        }
    }   
    if ident_lines.is_empty() {
        return 0;
    }
    //println!("{:?}",ident_lines);
    for line in ident_lines {
        let mut t = line.0;
        let mut b = line.1;
        loop {
            if p[t].iter().zip(&p[b]).filter(|&(a,b)| *a == *b).count() != p[t].len() {
                break;
            }
            if t == 0 || b == p.len()-1 {
                //println!("{} {}",t,b);
                return line.1;
            }
            //println!("{} {}",t,b);
            t -= 1;
            b += 1;
        }
    }
    0
}

fn find_vert_symmetry(p: &Vec<Vec<char>>) -> usize {
    let mut ident_lines: Vec<(usize,usize)> = vec![];
    let len = p[0].len();
    for x1 in 0..len-1 {
        if p.iter().zip(p).filter(|(a,b)| a[x1] == b[x1+1]).count() == p.len() {
            ident_lines.push((x1,x1+1));
        }
    }   
    if ident_lines.is_empty() {
        return 0;
    }
    //println!("{:?}",ident_lines);
    for line in ident_lines {
        let mut l = line.0;
        let mut r = line.1;
        loop {
            if p.iter().zip(p).filter(|(a,b)| a[l] == b[r]).count() != p.len() {
                break;
            }
            if l == 0 || r == len-1 {
                //println!("{} {}",l,r);
                return line.1;
            }
            //println!("{} {}",l,r);
            l -= 1;
            r += 1;
        }
    }
    0
}

fn find_hsmudge(p: &Vec<Vec<char>>, hs: usize) -> usize {
    let mut ident_lines: Vec<(usize,usize)> = vec![];    
    for y1 in 0..p.len()-1 {
        let cnt_ident = p[y1].iter().zip(&p[y1+1]).filter(|&(a,b)| *a == *b).count();
        if  ( cnt_ident == p[y1].len()-1 || cnt_ident == p[y1].len() ) && y1+1 != hs {
            ident_lines.push((y1,y1+1));            
        }
    }   
    if ident_lines.is_empty() {
        return 0;
    }
    //println!("{:?}",ident_lines);
    for line in ident_lines {
        let mut t = line.0;
        let mut b = line.1;
        let mut smudge_found = false;
        loop {
            let cnt = p[t].iter().zip(&p[b]).filter(|&(a,b)| *a == *b).count();
            if cnt == p[t].len() - 1 && !smudge_found {
                smudge_found = true;
            } else if cnt != p[t].len() {
                break;
            }
            if t == 0 || b == p.len()-1 {
                //println!("{} {}",t,b);
                return line.1;
            }
            //println!("{} {}",t,b);
            t -= 1;
            b += 1;
        }
    }
    0
}

fn find_vsmudge(p: &Vec<Vec<char>>, vs: usize) -> usize {
    let mut ident_lines: Vec<(usize,usize)> = vec![];
    let len = p[0].len();
    for x1 in 0..len-1 {
        let cnt_ident = p.iter().zip(p).filter(|(a,b)| a[x1] == b[x1+1]).count();
        if ( cnt_ident == p.len() || cnt_ident == p.len()-1 ) && x1+1 != vs {
            ident_lines.push((x1,x1+1));
        }
    }   
    if ident_lines.is_empty() {
        return 0;
    }
    //println!("{:?}",ident_lines);
    for line in ident_lines {
        let mut l = line.0;
        let mut r = line.1;
        let mut smudge_found = false;
        loop {
            let cnt = p.iter().zip(p).filter(|(a,b)| a[l] == b[r]).count();
            if cnt ==  p.len() -1 && !smudge_found {
                smudge_found = true;
            } else if cnt != p.len() {
                break;
            }
            if l == 0 || r == len-1 {
                //println!("{} {}",l,r);
                return line.1;
            }
            //println!("{} {}",l,r);
            l -= 1;
            r += 1;
        }
    }
    0
}

fn part_1(filename: &str) {
    let patterns = parse(filename);
    let mut sum = 0;
    for p in patterns {
        sum += 100 * find_hor_symmetry(&p) + find_vert_symmetry(&p);
    }
    println!("Answer for part 1: {}",sum);
}

fn part_2(filename: &str) {
    let patterns = parse(filename);
    let mut sum = 0;
    for p in patterns {
        let hs = find_hor_symmetry(&p);
        let vs = find_vert_symmetry(&p);        
        println!("hs={hs} vs={vs}");            
        let hsn = find_hsmudge(&p, hs);
        let vsn = find_vsmudge(&p, vs);
        println!("hsn={hsn} vsn={vsn}");     
        sum += hsn * 100 + vsn;          
    }    
    println!("Answer for part 2: {}",sum);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
