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
            if fetch_height(grid, (i as i32 + 1, j as i32)) == current_height + 1 {
                elem.push(Dir::DOWN);
            } else if fetch_height(grid, (i as i32 - 1, j as i32)) == current_height + 1 {
                elem.push(Dir::UP);
            } else if fetch_height(grid, (i as i32, j as i32 + 1)) == current_height + 1 {
                elem.push(Dir::RIGHT);
            } else if fetch_height(grid, (i as i32, j as i32 - 1)) == current_height + 1 {
                elem.push(Dir::LEFT);
            } else {
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

fn move_loc_by_dir(loc: (i32, i32), dir: Dir) -> (i32, i32) {
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
        _ => {return loc;}
    }
}

fn trace_paths(flow_map: &FlowMap, loc: (i32, i32)) -> Vec<Vec<(i32, i32)>>{
    
    let all_paths_terminated: bool = false;
    let mut paths: Vec<Vec<(i32, i32)>> = Vec::new();
    let mut path = Vec::new();
    path.push(loc);
    paths.push(path);
    while !all_paths_terminated {
        for i in 0..paths.len() {
            let dirs: Vec<Dir> = flow_map.map[loc.0 as usize][loc.1 as usize].clone();
        }
    }
    return paths;
}

fn trace_flow_map(flow_map: &FlowMap) {

}
fn main() {
    let path = Path::new("inputs/day10.txt");
    let grid = parse_file_to_grid(path);

    let flowmap = build_flow_map(&grid);

    
}