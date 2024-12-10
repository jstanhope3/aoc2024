extern crate aoc2024;
use aoc2024::{parse_file_to_grid, Grid};
use std::{path::Path};

#[derive(Clone, PartialEq)]
enum Dir {
    STOP,
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

struct FlowMap {
    map: Vec<Vec<Vec<Dir>>>
}

fn fetch_height(grid: &Grid, loc: (i32, i32)) -> i32 {
    if loc.0 >= 0 && loc.0 < grid.num_rows as i32 && loc.1 >= 0 && loc.1 < grid.num_cols  as i32 {
        return grid.cells[loc.0 as usize][loc.1 as usize].to_digit(10).unwrap() as i32;
    } else {
        return -1;
    }
}

fn build_flow_map(grid: &Grid) -> FlowMap {
    let mut vec: Vec<Vec<Vec<Dir>>> = Vec::new();
    for i in 0..grid.cells.len() {
        let mut row: Vec<Vec<Dir>> = Vec::new();
        for j in 0..grid.cells[i].len() {
            let mut elem: Vec<Dir> = Vec::new();
            let current_height: i32 = grid.cells[i][j].to_digit(10).unwrap() as i32;
            let mut is_end = true;
            if fetch_height(grid, (i as i32 + 1, j as i32)) == current_height + 1 {
                elem.push(Dir::DOWN);
                is_end = false;
            }
            if fetch_height(grid, (i as i32 - 1, j as i32)) == current_height + 1 {
                elem.push(Dir::UP);
                is_end = false;
            }
            if fetch_height(grid, (i as i32, j as i32 + 1)) == current_height + 1 {
                elem.push(Dir::RIGHT);
                is_end = false;
            }
            if fetch_height(grid, (i as i32, j as i32 - 1)) == current_height + 1 {
                elem.push(Dir::LEFT);
                is_end = false;
            }
            if is_end {
                elem.push(Dir::STOP);
            }
            row.push(elem);
        }
        vec.push(row);
    }
    FlowMap {
        map: vec
    }
}

fn move_loc_by_dir(loc: &(i32, i32), dir: &Dir) -> (i32, i32) {
    match dir {
        Dir::DOWN => {
            return (loc.0 + 1, loc.1);
        },
        Dir::UP => {
            return (loc.0 - 1, loc.1);
        },
        Dir::RIGHT => {
            return (loc.0, loc.1 + 1);
        },
        Dir::LEFT => {
            return (loc.0, loc.1 - 1);
        },
        _ => {return loc.clone();}
    }
}

fn trace_paths(flow_map: &FlowMap, loc: (i32, i32)) -> Vec<Vec<(i32, i32)>>{
    
    let mut all_paths_terminated: bool = false;
    let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut terminated_paths: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut path = Vec::new();
    path.push(loc);
    paths.push(path);
    let mut iter = 0;
    while !all_paths_terminated {
        all_paths_terminated = true;
        for i in 0..paths.len() {
            let current_loc = paths[i].last().unwrap().clone();
            let dirs: Vec<Dir> = flow_map.map[current_loc.0 as usize][current_loc.1 as usize].clone();
            if dirs.len() == 1 && dirs[0] == Dir::STOP {
                println!("path terminated");
                terminated_paths.push(paths[i].clone());
                paths.remove(i as usize);
                continue; // path is terminated, continue;
            }
            if dirs.len() == 1 {
                println!("updating path");
                all_paths_terminated = false;
                paths[i].push(move_loc_by_dir(&current_loc, &dirs[0]));
            } else {
                assert!(dirs.len() > 1); // sanity check
                println!("branch detected");
                let currpath = paths[i].clone();
                paths[i].push(move_loc_by_dir(&current_loc, &dirs[0]));
                for dir_i in 1..dirs.len() {
                    let mut new_path = currpath.clone();
                    new_path.insert(0, move_loc_by_dir(&current_loc, &dirs[dir_i]));
                    println!("path inserted");
                    paths.insert(i + 1, new_path);
                }
                all_paths_terminated = false;           
            }
        }
        iter += 1;
        println!("iter: {}", iter);
    }
    return paths;
}

fn score_paths(paths: &Vec<Vec<(i32, i32)>>, grid:&Grid ) -> i32 {
    let mut sum = 0;
    for path in paths {
        let last_pos = path.last().unwrap();
        if fetch_height(grid, *last_pos) == 9 {
            sum += 1;
        }
    }
    return sum;
}

// fn trace_flow_map(flow_map: &FlowMap) {
    
// }
fn main() {
    let path = Path::new("inputs/day10.txt");
    let grid = parse_file_to_grid(path);

    let flowmap = build_flow_map(&grid);
    let mut sum = 0;
    for i in 0..grid.cells.len() {
        for j in 0..grid.cells[i].len() {
            if grid.cells[i][j] == '0' {
                let pos = (i as i32, j as i32);
                let paths = trace_paths(&flowmap, pos);
                sum += score_paths(&paths, &grid);
            }
        }
    }
    println!("score of paths: {}", sum);
}