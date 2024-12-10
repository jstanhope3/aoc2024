extern crate aoc2024;
use aoc2024::read_file_to_string;
use std::{path::Path};

#[derive(Clone)]
struct MemBlock {
    len: usize,
    is_mem: bool,
    id: u32,
}
fn main() {
    let path = Path::new("inputs/day09.txt");
    let s = read_file_to_string(path);
    let chars: Vec<char> = s.trim().chars().collect();
    let mut mem_orig: Vec<i64> = Vec::new();
    let mut current_id: i64 = 0;
    let mut size_of_used_mem = 0;
    let mut mem_layout: Vec<MemBlock> = Vec::new();
    for i in 0..chars.len() {
        if i % 2 == 0 {
            // add x mem blocks of particular size
            let amt: u32 = chars[i].to_digit(10).unwrap();
            for _ in 0..amt {
                mem_orig.push(current_id);
            }
            size_of_used_mem += amt;
            mem_layout.push(
                MemBlock {
                    len: amt as usize,
                    is_mem: true,
                    id: current_id as u32,
                }
            );
            current_id += 1;
        } else {
            // add x mem blocks of empty space
            let amt: u32 = chars[i].to_digit(10).unwrap();
            for _ in 0..amt {
                mem_orig.push(-1);
            }
            mem_layout.push(
                MemBlock {
                    len: amt as usize,
                    is_mem: false,
                    id: 0,
                }
            );
        }
    }
    println!("This block of memory is {} long and has {} unique blocks", mem_orig.len(), current_id);
    // combine empty spaces
    println!("Part 1:");
    let mut mem = mem_orig.clone();
    let mut forward_idx: usize = 0;
    let mut backward_idx: usize = mem.len() - 1;
    while (forward_idx < mem.len()) {
        if forward_idx >= backward_idx || forward_idx >= size_of_used_mem as usize{
            println!("break");
            break;
        }
        if mem[forward_idx] == -1 { // empty space detected
            // find mem at the back of the buffer and move it to the front
            while (mem[backward_idx] == -1) {
                backward_idx -= 1;
            }
            mem[forward_idx] = mem[backward_idx];
            mem[backward_idx] = -1;
            // println!("{:?}", mem);
            
        } else {
            forward_idx += 1;
        }
    }
    // println!("{:?}", mem);
    // calculate filesystem checksum
    let mut sum:u64 = 0;
    for i in 0..mem.len() {
        if mem[i] == -1 {
            continue;
        }
        sum += i as u64 * mem[i] as u64;
    }
    println!("the check sum is: {}", sum);
    println!("Part 2:");
    // let mut mem = mem_orig.clone();
    // let mut mem_layout: Vec<u32> = Vec::new();
    // for c in chars.iter() {
    //     mem_layout.push(
    //         c.to_digit(10).unwrap()
    //     );
    // }
    let mut mem_layout_len = mem_layout.len();
    let mut curr_indx = mem_layout_len - 1;
    let mut curr_id = current_id - 1;
    while curr_indx > 0 {
        /*
        1. start at the block of memory at the back, keep track of its index
        2. find the left most empty space that this block fits into and move this block there
        3. replace existing layout with empty space
        4. repeat
         */
        if mem_layout[curr_indx].is_mem && mem_layout[curr_indx].id as i64 <= curr_id {
            curr_id = mem_layout[curr_indx].id as i64;
            for i in 0..curr_indx {
                if !mem_layout[i].is_mem && mem_layout[i].len >= mem_layout[curr_indx].len {
                    println!("moving block {curr_id}", );
                    let  tmp: MemBlock = mem_layout[curr_indx].clone();
                    mem_layout[curr_indx].is_mem = false;
                    mem_layout[i].len -= tmp.len;
                    mem_layout.insert(i, tmp);
                    break;
                }
            }
            // decrement curr_id to not repeat blocks
            curr_id -= 1;
        }


        // the last thing we do is decrement curr_indx
        curr_indx -= 1;
    }

    let mut mem_new: Vec<i64> = Vec::new();
    for memblock in mem_layout {
        for _ in 0..memblock.len {
            if memblock.is_mem {
                mem_new.push(memblock.id as i64);
            } else {
                mem_new.push(-1);
            }
        }
    }
    // println!("{:?}", mem_new);
    // calculate checksum
    let mut sum:u64 = 0;
    for i in 0..mem_new.len() {
        if mem_new[i] == -1 {
            continue;
        }
        sum += i as u64 * mem_new[i] as u64;
    }
    println!("the check sum is: {}", sum);

}