use std::fs::read_to_string;
use std::cmp;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn is_symbol(grid: &Vec::<Vec<char>>, r: usize, c: usize) -> bool {
    return !grid[r][c].is_numeric() && grid[r][c] != '.';
}

fn main() {
    let mut grid = Vec::<Vec<char>>::new();
    for line in read_lines("./input.txt") {
        let row = line.chars().collect();
        grid.push(row);
    }

    let mut i: usize = 0;
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut sum: i32 = 0;
    for row in &grid {
        let mut number_string = String::from("");
        let mut j: usize = 0;
        for cell in row {
            if cell.is_numeric() {
                number_string.push(*cell);
            } else {
                if !number_string.is_empty() {
                    let mut has_adjacent_symbol = false;

                    let mut j_start: usize = 0;
                    if j - number_string.len() > 0 {
                        j_start = j - number_string.len() - 1;
                    }

                    let j_end: usize = j + 1;

                    if i > 0 {
                        // look for symbols in row above
                        for x in j_start..j_end {
                            if is_symbol(&grid, i - 1, x) {
                                has_adjacent_symbol = true;
                            }
                        }
                    }

                    if i < num_rows - 1 {
                        // look for symbols in the row below
                        for x in j_start..j_end {
                            if is_symbol(&grid, i + 1, x) {
                                has_adjacent_symbol = true;
                            }
                        }
                    }

                    // look for symbol to the left
                    if j - number_string.len() > 0 {
                        if is_symbol(&grid, i, j - number_string.len() - 1) {
                            has_adjacent_symbol = true;
                        }
                    }

                    // look for symbol to the right
                    if j < num_cols - 1 {
                        if is_symbol(&grid, i, j) {
                            has_adjacent_symbol = true;
                        }
                    }

                    if has_adjacent_symbol {
                        // println!("{} has an adjacent symbol", number_string);
                        sum = sum + number_string.parse::<i32>().unwrap();
                    } else {
                        println!("{} does not have an adjacent symbol", number_string);
                    }

                    number_string = String::from("");
                }
                
            }

            j = j + 1
        }

        if !number_string.is_empty() {
            let mut has_adjacent_symbol = false;

            let mut j_start: usize = 0;
            if j - number_string.len() > 0 {
                j_start = j - number_string.len() - 1;
            }
            let j_end: usize = j;
            
            if i > 0 {
                // look for symbols in row above
                for x in j_start..j_end {
                    if is_symbol(&grid, i - 1, x) {
                        has_adjacent_symbol = true;
                    }
                }
            }

            if i < num_rows - 1 {
                // look for symbols in the row below
                for x in j_start..j_end {
                    if is_symbol(&grid, i + 1, x) {
                        has_adjacent_symbol = true;
                    }
                }
            }

            // look for symbol to the left
            if j - number_string.len() > 0 {
                if is_symbol(&grid, i, j - number_string.len() - 1) {
                    has_adjacent_symbol = true;
                }
            }

            if has_adjacent_symbol {
                sum = sum + number_string.parse::<i32>().unwrap();
            }
        }

        i = i + 1;
    }

    println!("answer is {}", sum);
}
