use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

const FILE_NAME: &str = "burma14.tsp";

#[derive(Debug)]
struct Node {
    id: i32,
    x: f64,
    y: f64,
}


fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open(FILE_NAME).unwrap();
    let reader = BufReader::new(file);

    let mut is_cood_section = false;
    for line in reader.lines() {
        let line = line?;
        if line.contains("NODE_COORD_SECTION") {
            is_cood_section = true;
            continue;
        }
        if is_cood_section {
            if line.contains("EOF") {
                break;
            }
            let record: Vec<&str> = line.split_whitespace().collect();
            let node = Node {
                id: record[0].parse()?,
                x: record[1].parse()?,
                y: record[2].parse()?,
            };
            println!("{:?}", node);
        }
    }
    Ok(())
}
