use std::fs;

struct MapItem {
    source : i64,
    target : i64,
    count : i64
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn map_seed_to_loc(seed: i64, maps: &Vec<Vec<MapItem>>) -> i64 {
    let mut tgt = seed;
    for map_cat in maps {
        for item in map_cat {
            if tgt >= item.source && tgt <= item.source + item.count - 1 {
                tgt = tgt + item.target - item.source;
                break;
            }
        }
    }
    return tgt;
}

fn parse(filename: &str) -> (Vec<i64>, Vec<Vec<MapItem>>) {
    let lines = read_file(filename);
    let seeds: Vec<i64> = lines[0][6..].split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut maps: Vec<Vec<MapItem>> = vec![];
    let mut map_cat: Vec<MapItem> = vec![];
    let mut skip_header = true;
    for line in lines.iter().skip(2) {
        if line.is_empty() {
            if !map_cat.is_empty() { maps.push(map_cat);};
            map_cat = vec![];
            skip_header = true;
            continue;
        }
        if skip_header {
            skip_header = false;
            continue;
        }
        let nums: Vec<i64> = line.split_ascii_whitespace().map(|x| x.parse().unwrap()).collect();
        map_cat.push(MapItem { target: nums[0], source: nums[1], count: nums[2] });        
    }
    maps.push(map_cat);
    (seeds, maps)
}

fn part_1(filename: &str){
    let (seeds,maps) = parse(filename);
    let mut loc_num = i64::MAX;
    for seed in seeds {
        let loc_tgt = map_seed_to_loc(seed, &maps);
        if loc_num > loc_tgt { loc_num = loc_tgt; }
    }
    println!("Answer for part 1 is {}", loc_num);
}

fn part_2(filename: &str){   
    let (seeds,maps) = parse(filename);
    let mut loc_num = i64::MAX;
    for (i,seed) in seeds.iter().enumerate() {
        if i % 2 != 0{ continue; };
        for j in *seed..*seed+seeds[i+1] {
            let loc_tgt = map_seed_to_loc(j, &maps);
            if loc_num > loc_tgt { loc_num = loc_tgt; }
        }
    }
    println!("Answer for part 1 is {}", loc_num);
}

fn main() {
    part_1("puzzle_input.txt");
    part_2("puzzle_input.txt");
}
