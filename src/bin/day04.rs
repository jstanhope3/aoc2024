extern crate aoc2024;
use aoc2024::read_file_to_string;
use std::path::Path;
use std::fmt::Display;

struct Square {
    data: Vec<Vec<char>>,
}

// builds a 7x7 square of characters from an index and the array:
fn get_square(i: i32, j: i32, s: i32, arr: &Vec<Vec<char>>) -> Square {
    let mut square: Square = Square{
        data: Vec::new(),
    };
    let h = (s / 2);
    for k in i - h..i + h + 1 {
        let mut row: Vec<char> = Vec::new();
        for l in j - h..j + h + 1 {
            if k < 0 || l < 0 {
                row.push('.');
            } else if k >= arr.len() as i32 || l >= arr[0].len() as i32 {
                row.push('.')
            } else {
                // println!("i: {}, j: {}", k, l);
                row.push(
                    arr[k as usize][l as usize]
                );
            }
        }
        square.data.push(row);
    }
    return square
}

fn count_xmas_in_square(sq: &Square) -> i32 {
    let offset_i = 3;
    let offset_j = 3;
    let dirs: [(i32, i32); 8] = [(0, 1), (1, 1), (1, 0), (1, -1), (-1, 1), (0, -1), (-1, 0), (-1, -1)];
    // there are 8 cases
    let pattern: [char; 3] = ['M', 'A', 'S'];
    let mut count = 0;
    for dir in dirs {
        let mut matches: bool = true;
        for n in 1..4 {
            let c: char = sq.data[(offset_i + dir.0 * n) as usize][(offset_j + dir.1 * n) as usize];
            let cn = pattern[(n - 1) as usize];
            matches = c == cn;
            if !matches {
                break; 
            }
        }
        if matches {
            count += 1;
        }
    }
    return count;
}

fn count_x_mas_in_square(sq: &Square) -> i32 {
    let offset_i = 1;
    let offset_j = 1;

    if sq.data[0][0] == 'M' && sq.data[0][2] == 'M' && sq.data[2][0] == 'S' && sq.data[2][2] == 'S' {
        return 1;
    }
    if sq.data[0][0] == 'M' && sq.data[2][0] == 'M' && sq.data[0][2] == 'S' && sq.data[2][2] == 'S' {
        return 1;
    }
    if sq.data[0][0] == 'S' && sq.data[0][2] == 'S' && sq.data[2][0] == 'M' && sq.data[2][2] == 'M' {
        return 1;
    }
    if sq.data[0][0] == 'S' && sq.data[2][0] == 'S' && sq.data[0][2] == 'M' && sq.data[2][2] == 'M' {
        return 1;
    }

    // if sq.data[0][1] == 'M' && sq.data[1][0] == 'M' && sq.data[2][1] == 'S' && sq.data[1][2] == 'S' {
    //     return 1;
    // }
    // if sq.data[0][1] == 'M' && sq.data[1][2] == 'M' && sq.data[2][1] == 'S' && sq.data[1][0] == 'S' {
    //     return 1;
    // }
    // if sq.data[0][1] == 'S' && sq.data[1][0] == 'S' && sq.data[2][1] == 'M' && sq.data[1][2] == 'M' {
    //     return 1;
    // }
    // if sq.data[0][1] == 'S' && sq.data[1][2] == 'S' && sq.data[2][1] == 'M' && sq.data[1][0] == 'M' {
    //     return 1;
    // }

    return 0;
}

impl Display for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.data.len() {
            for j in 0..self.data[i].len() {
                print!("{} ", self.data[i][j]);
            }
            print!("\n");
        }
        Ok(())
    }
}

fn main() {
    let path: &Path = Path::new("inputs/day04.txt");
    let mut input = read_file_to_string(path);

//     input = 
// ".M.S......
// ..A..MSMS.
// .M.S.MAA..
// ..A.ASMSM.
// .M.S.M....
// ..........
// S.S.S.S.S.
// .A.A.A.A..
// M.M.M.M.M.
// ..........".to_string();


    // first split input into lines with \n, then split each line into a vector of chars
    let lines: Vec<&str> = input.split("\n").collect();
    let mut arr: Vec<Vec<char>> = Vec::new();
    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        if line.len() == 0 {
            continue;
        }
        arr.push(chars);
    }
    println!("Size: {} x {}", arr.len(), arr[0].len());

    let mut count = 0;
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            let c = arr[i][j];
            if c != 'X' {
                continue;
            }
            // println!("i: {}, j: {}", i, j);
            let square = get_square(i as i32, j as i32,7, &arr);
            let local_count = count_xmas_in_square(&square);
            
            // if local_count > 0 {
            //     println!("Local count: {}", local_count);
            //     println!("{}", square);
            // }
            count += local_count;
            
        }
    }
    // assert_eq!(count, 2397);
    println!("Count 1: {}", count);

    let sq1 = get_square(1, 1, 3, &arr);
    println!("{}", sq1);

    // part 2
    count = 0;
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            let c = arr[i][j];
            if c != 'A' {
                continue;
            }
            // println!("i: {}, j: {}", i, j);
            let square = get_square(i as i32, j as i32,3, &arr);
            let local_count = count_x_mas_in_square(&square);
            // println!("{}", square);
            // println!("{}", local_count);
            // println!("-----------------------");
            // if local_count > 0 {
            //     println!("Local count: {}", local_count);
            //     println!("{}", square);
            // }
            count += local_count;
            
        }
    }
    println!("Task 2: {}", count);
}