extern crate aoc2024;
use aoc2024::read_file_to_string;
use std::{cell::Cell, path::Path};

#[derive(Copy, Clone, PartialEq)]
enum CellState{
    EMPTY,
    VISITED,
    OBSTACLE,
}


#[derive(Debug, Copy, Clone, PartialEq)]
enum Direction{
    UP, 
    DOWN,
    LEFT,
    RIGHT
}

#[derive(Clone, PartialEq)]
struct Map {
    pos: (i32, i32),
    size: (i32, i32),
    map: Vec<Vec<CellState>>,
    dir: Direction,
}

fn is_agent_in_map(map: &Map) -> bool {
    // println!("checking {} vs {}", map.pos.0, map.size.0);
    // println!("checking {} vs {}", map.pos.1, map.size.1);
    return !(map.pos.0 < 0 || map.pos.0 >= map.size.0 || map.pos.1 < 0 || map.pos.1 >= map.size.1);
}

fn fetch_cell_state(map: &Map, pos: (i32, i32)) -> CellState {
     if (pos.0 < 0 || pos.0 >= map.size.0 || pos.1 < 0 || pos.1 >= map.size.1) {
        return CellState::EMPTY; // out of bounds
     } else {
        return map.map[pos.0 as usize][pos.1 as usize];
     }
}

fn count_visited_cells(map: &Map) -> i32 {
    let mut count = 0;
    for i in 0..map.size.0 {
        for j in 0..map.size.1 {
            if map.map[i as usize][j as usize] == CellState::VISITED {
                count += 1;
            }
        }
    }
    return count;
}

fn step(map: &mut Map) {
    // query next cell
    let next_cell_pos: (i32, i32) = match map.dir {
        Direction::UP => {(map.pos.0 - 1, map.pos.1)},
        Direction::DOWN => {(map.pos.0 + 1, map.pos.1)},
        Direction::LEFT => {(map.pos.0, map.pos.1 - 1)},
        Direction::RIGHT => {(map.pos.0, map.pos.1 + 1)},
    };
    let next_cell_state = fetch_cell_state(map, next_cell_pos);
    match next_cell_state {
        CellState::OBSTACLE => {
            match map.dir {
                Direction::UP => {map.dir = Direction::RIGHT},
                Direction::DOWN => {map.dir = Direction::LEFT},
                Direction::LEFT => {map.dir = Direction::UP},
                Direction::RIGHT => {map.dir = Direction::DOWN},
            };
            // println!("Rotating to {:?}", map.dir);
        },
        _ => { // Visited or Empty
            // println!("index is {},  {} out of {}, {}", map.pos.0, map.pos.1, map.size.0, map.size.1);
            map.map[map.pos.0 as usize][map.pos.1 as usize] = CellState::VISITED; // set current cell to visited
            map.pos = next_cell_pos; // update position
            // println!("Step to {}, {}", map.pos.0, map.pos.1);
        },
    }
    // return map.map[map.pos.0 as usize][map.pos.1 as usize];
    
}

fn add_obstacle(map: &mut Map, i: usize, j: usize) {
    map.map[i][j] = CellState::OBSTACLE;
}

fn solve_map(map: &mut Map) -> i32 {
    let mut steps = 0;
    let mut loop_detection_count = 0;
    while is_agent_in_map(&map) {
        steps += 1;
        // let prev_map = map.clone();
        // println!("Step: {}", steps);
        step(map);
        let state = fetch_cell_state(map, map.pos);
        if state == CellState::VISITED {
            loop_detection_count +=1;
        } else {
            loop_detection_count = 0;
        }
        if loop_detection_count > map.size.0 {
            return -1;
        }
        // println!("End of step")
    }
    return steps;
}

fn main() {
    let path = Path::new("inputs/day06.txt");
    let s = read_file_to_string(path);
    let lines: Vec<&str> = s.split("\n").collect();
    let num_rows = lines.len() - 1;
    let num_cols = lines[0].len() - 1;
    let mut map: Map = Map {
        pos: (0,0),
        size: (num_rows as i32, num_cols as i32),
        map: Vec::with_capacity(num_rows),
        dir: Direction::UP,
    };
    println!("Loading map with {} rows and {} cols", num_rows, num_cols);
    // initialize map
    for _ in 0..num_rows {
        let mut v: Vec<CellState> = Vec::new();
        for _ in 0..num_cols {
            v.push(CellState::EMPTY);
        }
        assert_eq!(v.len(), num_cols);
        map.map.push(v);
    }
    assert_eq!(num_rows, map.map.len());
    for i in 0..lines.len() {
        let line = lines[i];
        let cs = line.chars().collect::<Vec<char>>();
        for j in 0..cs.len() {
            let c = cs[j];
            match c {
                '.' => { map.map[i][j] = CellState::EMPTY; },
                '#' => { map.map[i][j] = CellState::OBSTACLE; },
                '^' => {
                        map.map[i][j] = CellState::VISITED;
                        map.pos = (i as i32, j as i32);
                        map.dir = Direction::UP;
                        },
                'v' => {
                        map.map[i][j] = CellState::VISITED;
                        map.pos = (i as i32, j as i32);
                        map.dir = Direction::DOWN;
                        },
                '>' => {
                        map.map[i][j] = CellState::VISITED;
                        map.pos = (i as i32, j as i32);
                        map.dir = Direction::RIGHT;
                        },
                '<' => {
                        map.map[i][j] = CellState::VISITED;
                        map.pos = (i as i32, j as i32);
                        map.dir = Direction::LEFT;
                        },
                '\n' => {},
                '\t' => {},

                _ => { }
            }
        }
    }
    println!("facing {:?} at position: {},{}", map.dir, map.pos.0, map.pos.1);
    let init_map = map.clone();
    solve_map(&mut map);
    println!("Agent out of bounds: {}, {}", map.pos.0, map.pos.1);
    println!("Number of visited cells: {}", count_visited_cells(&map));

    // part 2;
    let mut num_loop_obst = 0;
    let mut count = 0;
    for i in 0..init_map.size.0 {
        for j in 0..init_map.size.1 {
            count += 1;
            let mut current_map = init_map.clone();
            if (i, j) == init_map.pos { continue; }
            add_obstacle(&mut current_map, i as usize, j as usize); // add obstacle to the map
            if solve_map(&mut current_map) == -1 {
                println!("Looped {} / {}", count, 130 * 130);
                num_loop_obst += 1;
            } else {
                println!("Solved {} / {}", count, 130 * 130);
            }
            
        }
    }
    println!("number of tiles you can place an obstacle to create a loop: {}", num_loop_obst);
}