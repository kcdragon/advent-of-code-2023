use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let mut count: i32 = 0;
    for line in read_lines("./one.txt") {
        let mut numbers = Vec::<char>::new();
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c);
            }
        }
        
        let number_as_string = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        let number: i32 = number_as_string.parse().unwrap();
        count = count + number;
    }
    println!("count: {}", count);
}