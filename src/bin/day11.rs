extern crate aoc2024;
use aoc2024::read_file_to_string;
use core::num;
use std::{path::Path};
use std::collections::HashMap;

fn count_stones_in_hash_map(stones_count: &HashMap<u64, u64>) -> u64{
    let mut sum: u64 = 0;
    for key in stones_count.keys() {
        sum += stones_count[key];
    }
    return sum;
}

fn add_stone(stones_count: &mut HashMap<u64, u64>, stone_marking: u64, stone_num: u64) {
    stones_count.entry(stone_marking).and_modify(|count| *count += stone_num).or_insert(stone_num);
}
fn main() {
    let path = Path::new("inputs/day11.txt");
    let s = read_file_to_string(path);
    let nums_strs: Vec<&str> = s.split_whitespace().collect();
    let mut stones: Vec<u64> = Vec::new();

    // initial configuration
    for str in nums_strs.iter() {
        stones.push(str.trim().parse::<u64>().unwrap());
    }
    // the solution to part 1 is the brute force solution
    // keeping it in here so that the solution is still visible for posterity
    // but begins to get really slow after iteration 25... 
    let mut num_blinks = 25;
    for blink in 0..num_blinks {
        // iterate over stones and 
        let mut idx = 0;
        while idx < stones.len() {
            // 1st rule
            if stones[idx] == 0 {
                stones[idx] = 1;
            } else if stones[idx].to_string().len() % 2 == 0 {
                let big_num = stones[idx].to_string();
                stones[idx] = big_num.split_at(big_num.len() / 2).0.parse::<u64>().unwrap();
                stones.insert(idx + 1, big_num.split_at(big_num.len() / 2).1.parse::<u64>().unwrap());
                idx += 1;
            } else {
                stones[idx] *= 2024;
            }
            idx += 1;
        }
        println!("Blink: {}, Number of stones: {}", blink, stones.len());
    }
    println!("Final number of stones: {}", stones.len());

    println!("-------part 2-------");
    
    num_blinks = 75;
    // rebuild stones
    stones = Vec::new();

    // initial configuration
    for str in nums_strs {
        stones.push(str.trim().parse::<u64>().unwrap());
    }
    let mut stones_count: HashMap<u64, u64> = HashMap::new();
    // initial configuration
    for stone in stones {
        stones_count.insert(stone, 1);
    }
    println!("Initial: Number of stones: {}", count_stones_in_hash_map(&stones_count));
    for blink in 0..num_blinks {
        let mut new_stones_count: HashMap<u64, u64> = HashMap::new();
        for stone_marking in stones_count.keys() {
            let num_stones = stones_count[stone_marking];
            if *stone_marking == 0 {
                add_stone(&mut new_stones_count, 1, num_stones);
            } else if stone_marking.to_string().len() % 2 == 0 {
                let big_num = stone_marking.to_string();
                let stone_marking_0 = big_num.split_at(big_num.len() / 2).0.parse::<u64>().unwrap();
                let stone_marking_1 = big_num.split_at(big_num.len() / 2).1.parse::<u64>().unwrap();
                add_stone(&mut new_stones_count, stone_marking_0, num_stones);
                add_stone(&mut new_stones_count, stone_marking_1, num_stones);
            } else {
                add_stone(&mut new_stones_count, stone_marking * 2024, num_stones);
            }
        }
        stones_count = new_stones_count;
        println!("Blink: {}, Number of stones: {}", blink, count_stones_in_hash_map(&stones_count));
    }
    println!("Final number of stones: {}", count_stones_in_hash_map(&stones_count));
}