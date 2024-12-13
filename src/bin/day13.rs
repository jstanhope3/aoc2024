extern crate aoc2024;
use aoc2024::read_file_to_string;
use core::num;
use std::{path::Path};

#[derive(Clone)]
struct LinearSystem {
    ax: i128,
    bx: i128,
    ay: i128,
    by: i128,
    prizex: i128,
    prizey: i128,
}

fn parse_numbers_from_line(s: &str)  -> (i128, i128) {
    let mut start: i128 = -1;
    let mut finish = 0;
    let chars: Vec<char> = s.chars().collect();
    let mut ret: (i128, i128) = (0, 0);
    for i in 0..s.len() + 1 {
        let mut c = '\0';
        if i < s.len() {
            c = chars[i];
        }
        if c >= '0' && c <= '9' {
            if start == -1 {
                start = i as i128;
            } else {
                finish = i as i128;
            }
        } else {
            if start > -1 && finish > 0 { // detected number
                let valstr  = &s.to_string()[start as usize..finish as usize + 1 ];
                let val = valstr.parse::<i128>().unwrap();
                if c == '\0' {
                    ret.1 = val;
                } else {
                    ret.0 = val;
                }
                start = -1;
                finish = 0;
            }
        }
    }
    return ret;
}

fn parse_string_to_linear_systems(s: &String) -> Vec<LinearSystem> {
    let mut vec: Vec<LinearSystem> = Vec::new();
    let mut nums: Vec<(i128, i128)> = Vec::new();
    let lines: Vec<&str> = s.split("\n").collect();
    let mut line_in_sys = 0;
    for i  in 0..lines.len() {
        let line = lines[i];
        if line.trim() == "" {
            continue;
        }
        let coeffs = parse_numbers_from_line(line);
        // println!("Coeffs: {:?}",coeffs);
        nums.push(coeffs);
    }

    let mut idx: usize = 0;
    while idx < nums.len() {
        let linsys = LinearSystem {
            ax: nums[idx].0,
            ay: nums[idx].1,
            bx: nums[idx + 1].0,
            by: nums[idx + 1].1,
            prizex: nums[idx + 2].0 + 10000000000000,
            prizey: nums[idx + 2].1 + 10000000000000,
        };
        println!("prizes: {}, {}", linsys.prizex, linsys.prizey);

        // increment i
        idx += 3;
        vec.push(linsys);
    }

    return vec;
}

// system is of form:
// ax * a + bx * b = prizex
// ay * a + by * b = prizey
fn solve_linear_system(sys_in: &LinearSystem) -> Option<(i128, i128)> {
    let mut linsys = sys_in.clone();
    // cancel out As
    linsys.ax *= sys_in.ay;
    linsys.bx *= sys_in.ay;
    linsys.prizex *= sys_in.ay;
    linsys.ay *= -sys_in.ax;
    linsys.by *= -sys_in.ax;
    linsys.prizey *= -sys_in.ax;
    // check that As are cancelled
    assert_eq!(0, linsys.ax + linsys.ay);
    assert_ne!(0, linsys.bx + linsys.by);
    let b = (linsys.prizex + linsys.prizey) as f64 / (linsys.bx + linsys.by) as f64;
    let a = (sys_in.prizex as f64 - sys_in.bx as f64 * b) / sys_in.ax as f64;

    if a.round() == a && b.round() == b  && a >= 0.0 && b >= 0.0 {
        println!("solution: {}, {}: valid", a, b);
        assert_eq!(a as i128 * sys_in.ax + b  as i128 * sys_in.bx, sys_in.prizex);
        assert_eq!(a as i128 * sys_in.ay + b  as i128 * sys_in.by, sys_in.prizey);
        return Some((a as i128, b as i128));
    } else {
        println!("solution: {}, {}: invalid", a, b);
        return None;
    }

}

fn main() {
    let path = Path::new("inputs/day13.txt");
    let s = read_file_to_string(path);
    let systems: Vec<LinearSystem> = parse_string_to_linear_systems(&s);

    let mut num_tokens = 0;
    for sys in systems.iter() {
        match solve_linear_system(sys) {
            None => {continue;},
            Some(sol) => {num_tokens += 3 * sol.0 + sol.1;}
        };
    }
    println!("num tokens needed: {}", num_tokens);
}