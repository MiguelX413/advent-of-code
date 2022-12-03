use std::path::Path;
use std::{env, fs};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    let file_path = Path::new(&args[1]);
    let file_contents = fs::read_to_string(file_path).unwrap();
    let max = file_contents
        .split("\n\n")
        .map(|f| {
            f.split_whitespace()
                .map(|f| f.trim().parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .max();

    if let Some(max) = max {
        println!("Max sum: {}", max);
    } else {
        println!("Max sum: None");
    }
}
