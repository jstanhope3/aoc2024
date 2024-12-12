extern crate aoc2024;
use aoc2024::{parse_file_to_grid, Grid, fetch_grid_val};
use std::{path::Path};

fn main() {
    let path = Path::new("inputs/day12_hint.txt");
    let grid = parse_file_to_grid(path);

    let regions: Vec<(char, Vec<(i32, i32)>)> = Vec::new();

    for i in 0..grid.num_rows {
        for j in 0 ..grid.num_cols {
            let pos = (i as i32, j as i32);
            let plant = fetch_grid_val(&grid, pos);
            
        }
    }
}