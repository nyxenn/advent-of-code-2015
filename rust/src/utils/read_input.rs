use std::fs::read_to_string;

pub fn read_input(days: u8, part: u8) -> String {
    let filepath = format!("./input/day_{:0>2}_part_{}.txt", days, part);
    println!("Reading input from filepath: {}", filepath);
    read_to_string(filepath).unwrap()
}
