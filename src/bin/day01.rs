use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn parse_line(s: &String) -> (i32, i32) {
    let parts: Vec<&str> = s.split("   ").collect();
    let a: &str = parts[0];
    let b: &str = parts[1];
    (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())    
}

fn main() {
    let path = Path::new("inputs/day01.txt");
    let mut v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(&path) {
        for line in lines.flatten() {
            let nums: (i32, i32) = parse_line(&line);
            v1.push(nums.0);
            v2.push(nums.1)
        }
    };
    
    // compute distance
    v1.sort();
    v2.sort();
    let mut dist = 0;
    for i in 0..v1.len() {
        dist += (v1[i] - v2[i]).abs();
    }
    println!("Total distance: {}", dist);

    // compute similarity score
    let mut score: i32 = 0;
    for i in 0..v1.len() {
        let num: i32 = v1[i];
        let mut num_matches: i32 = 0;
        for j in 0..v2.len() {
            if v2[j] == num {
                num_matches += 1;
            }
        }
        score += num_matches * num;
    }
    println!("Similarity score: {}", score)
}