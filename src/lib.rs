use std::fmt::Display;
use std::fs::File;
use std::path::Path;
use std::io::Read;

#[derive(Clone)]
pub struct Grid {
    pub cells: Vec<Vec<char>>,
    pub num_rows: usize,
    pub num_cols: usize,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        println!("Grid with size {} x {}", self.num_rows, self.num_cols);
        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                print!("{} ", self.cells[i][j]);
            }
            print!("\n");
        }
        Ok(())
    }
}

pub fn read_file_to_string(path: &Path) -> String {
    let display = path.display();
    // Open the path in read-only mode, returns `io::Result<File>`
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    // Read the file contents into a string, returns `io::Result<usize>`
    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => {},
    }
    return s;
}

pub fn parse_file_to_grid(path: &Path) -> Grid {
    let s = read_file_to_string(path);
    let lines: Vec<&str> = s.split("\n").collect();
    let mut grid: Vec<Vec<char>> = Vec::new();
    for i in 0..lines.len() {
        let line = lines[i];
        if line.trim().len() == 0 {
            continue;
        }
        let cs = line.chars().collect::<Vec<char>>();
        let mut row: Vec<char> = Vec::new();
        for j in 0..cs.len() {
            row.push(cs[j]);
        }
        grid.push(row);
    }

    Grid {
        cells: grid.clone(),
        num_cols: grid.len(),
        num_rows: grid[0].len(),
    }
}