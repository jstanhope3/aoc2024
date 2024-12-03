use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// copied from day01
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn is_report_safe(v: &Vec<&str>) -> bool {
    let mut all_increasing: bool = true;
    let mut all_decreasing: bool = true;
    let mut at_least_one: bool = true;
    let mut at_most_three: bool = true;
    
    for i in 1..v.len() {
        let a: i32 = v[i].parse::<i32>().unwrap();
        let b: i32 = v[i - 1].parse::<i32>().unwrap();
        if a > b { all_decreasing = false; }
        if a < b { all_increasing = false; }
        if  (a - b).abs() < 1 { at_least_one = false;}
        if  (a - b).abs() > 3 { at_most_three = false;}
    }

    (all_increasing || all_decreasing) && (at_most_three && at_least_one)
}

fn validate_report(s: &String, use_dampener: bool) -> bool {
    // use dampener
    let v: Vec<&str> = s.split_whitespace().collect();
    if !use_dampener {
        return is_report_safe(&v);
    } else {
        if is_report_safe(&v) {
            return true;
        }
        // check permutations
        let mut num_unsafe_combinations = 0;
        let num_reports: usize = v.len();
        for i in 0..num_reports {
            let mut v2 = v.clone();
            v2.remove(i);
            if is_report_safe(&v2) {
                return true;
            }
        }
        return false;
    }
}  
fn main() {
    let mut safe_reports = 0;
    let path = Path::new("inputs/day02.txt");
    // part one
    if let Ok(lines) = read_lines(&path) {
        for line in lines.flatten() {
            if validate_report(&line, false) {
                safe_reports += 1;
            }
        }
    };
    println!("Num safe reports: {}", safe_reports);
    // part 2
    safe_reports = 0;
    if let Ok(lines) = read_lines(&path) {
        for line in lines.flatten() {
            if validate_report(&line, true) {
                safe_reports += 1;
            }
        }
    };
    println!("Num safe reports with dampener: {}", safe_reports);
}