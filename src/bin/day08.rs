extern crate aoc2024;
use aoc2024::{parse_file_to_grid, Grid};
use std::{path::Path};

#[derive(Copy, Clone)]
struct Antenna {
    freq: char,
    location: (i32, i32),
}

#[derive(Copy, Clone)]

struct AntiNode {
    freq: char,
    location: (i32, i32),
}

fn find_antennae(grid: &Grid) -> Vec<Antenna> {
    let mut antennae: Vec<Antenna> = Vec::new();
    for i in 0..grid.num_rows {
        for j in 0..grid.num_cols {
            match grid.cells[i][j] {
                '.' => {}
                _ => {
                    antennae.push(
                        Antenna {
                            freq: grid.cells[i][j],
                            location: (i as i32, j as i32)
                        }
                    )
                }
            }
        }
    }
    return antennae;
}

fn find_anti_nodes(antennae: &Vec<Antenna>, bounds: (i32, i32), cap_nodes: bool) -> Vec<AntiNode> {
    let mut nodes: Vec<AntiNode> = Vec::new();
    for i in 0..antennae.len() {
        for j in i+1..antennae.len() {
            let antenna1 = antennae[i];
            let antenna2 = antennae[j];
            if antenna1.freq != antenna2.freq { // check for matching frequencies
                continue;
            }
            let d = (antenna1.location.0 - antenna2.location.0, antenna1.location.1 - antenna2.location.1);
            // backwards direction
            let mut node_is_in_bounds: bool = true;
            let mut dist = 0;
            while(node_is_in_bounds) {
                
                let loc = (antenna1.location.0 + dist * d.0, antenna1.location.1 + dist * d.1);
                if loc.0 >= 0 && loc.0 <= bounds.0 && loc.1 >= 0 && loc.1 <= bounds.1 {
                    nodes.push(
                        AntiNode {
                            freq: antenna1.freq,
                            location: loc,
                        }
                    )
                } else {
                    node_is_in_bounds = false;
                }
                dist += 1;
                if cap_nodes {
                    break;
                }
            }
            node_is_in_bounds = true;
            dist = 0;
            while(node_is_in_bounds) {
                
                let loc = (antenna2.location.0 - dist * d.0, antenna2.location.1 - dist * d.1);
                if loc.0 >= 0 && loc.0 <= bounds.0 && loc.1 >= 0 && loc.1 <= bounds.1 {
                    nodes.push(
                        AntiNode {
                            freq: antenna1.freq,
                            location: loc,
                        }
                    )
                } else {
                    node_is_in_bounds = false;
                }
                dist += 1;
                if cap_nodes {
                    break;
                }
            }
        }
    }
    return nodes;
}

fn count_unique_anti_nodes(nodes: &Vec<AntiNode>) -> i32 {
    let mut locs: Vec<(i32, i32)> = Vec::new();
    for node in nodes {
        if !locs.contains(&node.location) {
            locs.push(node.location)
        }
    }
    locs.len() as i32
}

fn mark_anti_nodes_on_grid(grid: &mut Grid, nodes: &Vec<AntiNode>) {
    for node in nodes {
        if grid.cells[node.location.0 as usize][node.location.1 as usize] != '.' {
            continue;
        }
        grid.cells[node.location.0 as usize][node.location.1 as usize] = '#';
    }
}


fn main() {
    let path: &Path = Path::new("inputs/day08.txt");
    let char_grid = parse_file_to_grid(path);
    let antennae: Vec<Antenna> = find_antennae(&char_grid);
    let nodes = find_anti_nodes(&antennae, (char_grid.num_rows as i32 -1, char_grid.num_cols as i32-1), true);
    let mut new_grid = char_grid.clone();
    println!("Part 1: number of nodes: {}, number of unique antinodes {}", nodes.len(), count_unique_anti_nodes(&nodes));
    let nodes2 = find_anti_nodes(&antennae, (char_grid.num_rows as i32 -1, char_grid.num_cols as i32-1), false);
    mark_anti_nodes_on_grid(&mut new_grid, &nodes2);
    println!("{}", new_grid);
    println!("Part 2: number of nodes: {}, number of unique antinodes {}", nodes2.len(), count_unique_anti_nodes(&nodes2));
}