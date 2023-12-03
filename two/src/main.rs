use std::fs::read_to_string;

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename) 
        .unwrap()  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect()  // gather them together into a vector
}

fn main() {
    let mut game_id_sum: i32 = 0;
    for line in read_lines("./input.txt") {
        let collection = line.split(":").collect::<Vec<&str>>();

        let game_id_part = collection[0];
        let game_id_parts = game_id_part.split(" ").collect::<Vec<&str>>();
        let game_id: i32 = game_id_parts[1].parse().unwrap();

        let result_part = collection[1].trim();
        let sets = result_part.split(";");
        let mut is_possible = true;
        for set in sets {
            let trimmed_set = set.trim();
            let cubes = trimmed_set.split(", ");
            for cube in cubes {
                let cube_parts = cube.split(" ").collect::<Vec<&str>>();
                let number: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];
                

                // 12 red cubes, 13 green cubes, and 14 blue cubes
                if color == "red" {
                    // dbg!(number_string);
                    // dbg!(color);
                    if number > 12 {
                        is_possible = false;
                    }
                } else if color == "green" {
                    if number > 13 {
                        is_possible = false;
                    }
                } else {
                    if number > 14 {
                        is_possible = false;
                    }
                }
            }
            
        }

        if is_possible {
            game_id_sum = game_id_sum + game_id;
        }
    }
    println!("Answer: {}", game_id_sum);
}
