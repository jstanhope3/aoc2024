extern crate aoc2024;
use aoc2024::{parse_file_to_grid, Grid, fetch_grid_val};
use core::num;
use std::{path::Path};

type Region = (char, Vec<(i32, i32)>);
type Cell = (i32, i32);

fn is_cell_in_region(regions: &Vec<Region>, plant: char, cell: &Cell) -> bool {
    for region in regions {
        if region.0 == plant && region.1.contains(cell) {
            return true;
        }
    }
    return false;
}

fn count_fences(grid: &Grid, region: &Region) -> i32{
    let mut num_fences = 0;
    for cell in region.1.iter() {
        let mut peek_cell = (cell.0 + 1, cell.1);
        if fetch_grid_val(grid, peek_cell) != region.0  {
            num_fences += 1;
        }
        peek_cell = (cell.0 - 1, cell.1);
        if fetch_grid_val(grid, peek_cell) != region.0  {
            num_fences += 1;
        }
        peek_cell = (cell.0, cell.1 + 1);
        if fetch_grid_val(grid, peek_cell) != region.0  {
            num_fences += 1;
        }
        peek_cell = (cell.0, cell.1 - 1);
        if fetch_grid_val(grid, peek_cell) != region.0  {
            num_fences += 1;
        }
    }
    return num_fences
}

fn grow_region(grid: &Grid, region: &mut Region) {
    let mut is_finished = false;
    while !is_finished {
        is_finished = true;
        for cell in region.1.clone() {
            let mut peek_cell = (cell.0 + 1, cell.1);
            if fetch_grid_val(grid, peek_cell) == region.0 && !region.1.contains(&peek_cell) {
                is_finished = false;
                region.1.push(peek_cell);
            }
            peek_cell = (cell.0 - 1, cell.1);
            if fetch_grid_val(grid, peek_cell) == region.0 && !region.1.contains(&peek_cell) {
                is_finished = false;
                region.1.push(peek_cell);
            }
            peek_cell = (cell.0, cell.1 + 1);
            if fetch_grid_val(grid, peek_cell) == region.0 && !region.1.contains(&peek_cell) {
                is_finished = false;
                region.1.push(peek_cell);
            }
            peek_cell = (cell.0, cell.1 - 1);
            if fetch_grid_val(grid, peek_cell) == region.0 && !region.1.contains(&peek_cell) {
                is_finished = false;
                region.1.push(peek_cell);
            }
        }
    }
}

fn main() {
    let path = Path::new("inputs/day12.txt");
    let grid: Grid = parse_file_to_grid(path);

    let mut regions: Vec<Region> = Vec::new();

    for i in 0..grid.num_rows {
        for j in 0 ..grid.num_cols {
            let pos = (i as i32, j as i32);
            let plant = fetch_grid_val(&grid, pos);
            // check if this cell is already in a region:
            if is_cell_in_region(&regions, plant, &pos) {
                continue;
            } else {
                // create a new region and then grow it :)
                let mut vec = Vec::new();
                vec.push(pos);
                let mut region: Region = (plant, vec);
                grow_region(&grid, &mut region);
                regions.push(region);
            }
        }
    }
    let mut price = 0;
    for region in regions {
        let num_fences = count_fences(&grid, &region);
        let local_price: i32 = region.1.len() as i32 * num_fences;
        price += local_price;
        println!("A region of {} plants with price {} * {} = {}", region.0, region.1.len(), num_fences, local_price);
    }
    println!("total price is {}", price)
}