use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let mut points: u32 = 0;
    for line in read_lines("./input.txt") {
        let mut parts = line.split(":").collect::<Vec<&str>>();
        let numbers_part = parts[1];

        parts = numbers_part.split("|").collect::<Vec<&str>>();
        
        let winning_part = parts[0].trim();
        let winning_numbers: Vec<&str> = winning_part.split_whitespace().collect();
    
        let your_part = parts[1].trim();
        let your_numbers: Vec<&str> = your_part.split_whitespace().collect();

        let mut matches: u32 = 0;
        for w in &winning_numbers {
            for y in &your_numbers {
                if w.eq(y) {
                    matches = matches + 1;
                }
            }
        }

        // dbg!(winning_numbers);
        // dbg!(your_numbers);
        // dbg!(matches);

        if matches > 0 {
            points = points + 2_u32.pow(matches - 1);
            // dbg!(points);
        }
    }
    println!("answer: {}", points);
}
