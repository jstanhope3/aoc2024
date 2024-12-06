use std::path::Path;
extern crate aoc2024;
use aoc2024::read_file_to_string;
fn main() {
    let path = Path::new("inputs/day03.txt");
    let s: String = read_file_to_string(path);
    // s = String::from("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"); // example from webpage
    // s = String::from("xmul(2,4)&mul[3,7]!^do()don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"); // example 2 from webpage
    let parts: Vec<&str> = s.split("mul(").collect();
    let mut sum = 0;
    let mut enabled = true;
    let part2 = true;
    for p in parts {
        println!("{}", p);
        println!("-------------" );
        let subparts: Vec<&str> = p.split(",").collect();
        if !(subparts.len() < 2 || !p.contains(')')) {
            let mut a: i32 = 0;
            let mut b: i32 = 0;
            
            a = match &subparts[0].parse::<i32>() {
                Err(_) => 0,
                Ok(opt) => {println!("a: {}", opt); *opt}
            };
            
            let subparts2: Vec<&str> = subparts[1].split(")").collect();
            b = match &subparts2[0].parse::<i32>() {
                Err(_) => 0,
                Ok(opt) => {println!("b: {}", opt); *opt}
            };

            let pos = subparts[0].len() + subparts2[0].len() + 1;
            if p.chars().collect::<Vec<char>>()[pos] != ')' {
                println!("Pos: {}, {}", pos, p.chars().collect::<Vec<char>>()[pos]);
                a = 0;
                b = 0;
            }

            if a > 0 && b > 0 && enabled {
                println!("Valid: {}", a * b)
            } else if a > 0 && b > 0 {
                println!("not enabled: {}", a * b)
            }
            else {
                println!("Invalid: a: {}, b: {}", a, b)
            }
            if enabled {
                sum += a * b;
            }
        }       

        
        // check for do and dont afterward
        if part2 {
            if p.contains("do()") && p.contains("don't()") {
                if p.find("do()") > p.find("don't()") {
                    enabled = true;
                } else {
                    enabled = false;
                }
            } else if p.contains("do()") {
                enabled = true;
            } else if p.contains("don't()") {
                enabled = false;
            } else {
                enabled = enabled;
            }            
        }
        println!("{}", enabled);
        println!("-------------" );
    }
    println!("Sum: {}", sum)
}