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
        let mut word = String::from("");
        for c in line.chars() {
            if c.is_numeric() {
                numbers.push(c);
                word = String::from("");
            } else {
                word.push(c);
                
                if word.ends_with("one") {
                    numbers.push('1');
                } else if word.ends_with("two") {
                    numbers.push('2');
                } else if word.ends_with("three") {
                    numbers.push('3');
                } else if word.ends_with("four") {
                    numbers.push('4');
                } else if word.ends_with("five") {
                    numbers.push('5');
                } else if word.ends_with("six") {
                    numbers.push('6');
                } else if word.ends_with("seven") {
                    numbers.push('7');
                } else if word.ends_with("eight") {
                    numbers.push('8');
                } else if word.ends_with("nine") {
                    numbers.push('9');
                }
            }
        }
        
        let number_as_string = format!("{}{}", numbers[0], numbers[numbers.len() - 1]);
        let number: i32 = number_as_string.parse().unwrap();
        count = count + number;
    }
    println!("count: {}", count);
}
