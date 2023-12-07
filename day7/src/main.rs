use std::fs;
use std::cmp;

struct Hand {
    str : String,
    cards: Vec<u8>,
    typ : u8,    
    bid: u32,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse_cards(card: &str, use_joker: bool, joker_value: u8) -> Vec<u8> {
    let mut res: Vec<u8> = vec![];
    for c in card.chars() {
        if c.is_digit(10) {
            res.push(c.to_digit(10).unwrap() as u8);
        } else {
            let i = match c {
                'T' => 10,
                'J' => { if use_joker {joker_value} else {11} },
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!("Ooops!") };
            res.push(i);
        }
    }
    res
 }

fn get_typ_by_count(m: &Vec<(u8,u8)>) -> u8{
    let mut typ: u8 = 0;
    if m[0].1 == 5 {
        typ = 6; // Five of a kind
    } else if m[0].1 == 4 {
        typ = 5; // Four of a kind
    } else if  m[0].1 == 3 && m[1].1 == 2 {
        typ = 4; // Full house
    } else if  m[0].1 == 3 && m[1].1 == 1 {
        typ = 3; // Three of a kind
    } else if  m[0].1 == 2 && m[1].1 == 2 {
        typ = 2; // Two pair
    } else if  m[0].1 == 2 && m[1].1 == 1 {
        typ = 1; // One pair
    }
    typ
}

fn get_typ(cards: &str, use_joker: bool) -> u8 {
    let c = parse_cards(cards, use_joker, 1);
    let mut typ: u8 = 0; // High card
    let mut m: Vec<(u8,u8)> = vec![];
    for v in c {
        match m.iter().position(|(x,y)| *x == v) {
            Some(p) => { m[p].1 += 1; }
            None => { m.push((v,1)); }
        }
    }
    m.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    if !use_joker{
        return  get_typ_by_count(&m) 
    } else {
        let t1 = get_typ_by_count(&m);
        let joker = if m[0].0 != 1 || m.len() < 2 { m[0].0 } else { m[1].0 };
        m.clear();
        let c = parse_cards(cards, use_joker, joker);
        for v in c {
            match m.iter().position(|(x,y)| *x == v) {
                Some(p) => { m[p].1 += 1; }
                None => { m.push((v,1)); }
            }
        }        
        m.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let t2 = get_typ_by_count(&m);
        return cmp::max(t1,t2);
    }
 }

fn parse(filename: &str, use_joker: bool) -> Vec<Hand> {
    let mut res: Vec<Hand> = vec![];
    for line in read_file(filename) {
        let s: Vec<&str> = line.split_whitespace().collect();
        let h: Hand = Hand {
            str: s[0].to_string(),
            cards: parse_cards(s[0],use_joker,1),
            bid: s[1].parse().unwrap(),
            typ: get_typ(s[0],use_joker),
        };
        res.push(h);
    }
    res
}

fn part_1(filename: &str) {
    let mut hands = parse(filename,false);
    hands.sort_by(|a,b| a.typ.partial_cmp(&b.typ).unwrap()
                            .then_with(|| a.cards[0].cmp(&b.cards[0]))
                            .then_with(|| a.cards[1].cmp(&b.cards[1]))
                            .then_with(|| a.cards[2].cmp(&b.cards[2]))
                            .then_with(|| a.cards[3].cmp(&b.cards[3]))
                            .then_with(|| a.cards[4].cmp(&b.cards[4])));

    let mut ans = 0;                        
    for (rank, h) in hands.iter().enumerate() {
        ans += (rank as u32+1) * h.bid;
        //println!("Hand {} rank {} typ {}", h.str, rank + 1, h.typ);
    }
    println!("Answer for part 1: {}",ans);
}

fn part_2(filename: &str) {
    let mut hands = parse(filename,true);
    hands.sort_by(|a,b| a.typ.partial_cmp(&b.typ).unwrap()
                            .then_with(|| a.cards[0].cmp(&b.cards[0]))
                            .then_with(|| a.cards[1].cmp(&b.cards[1]))
                            .then_with(|| a.cards[2].cmp(&b.cards[2]))
                            .then_with(|| a.cards[3].cmp(&b.cards[3]))
                            .then_with(|| a.cards[4].cmp(&b.cards[4])));

    let mut ans = 0;                        
    for (rank, h) in hands.iter().enumerate() {
        ans += (rank as u32+1) * h.bid;
        //println!("Hand {} rank {} typ {}", h.str, rank + 1, h.typ);
    }
    println!("Answer for part 2: {}",ans);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
