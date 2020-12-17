use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};

fn read_input_data(file_name: &str) -> Vec<Vec<u8>> {
    let f = File::open(file_name).unwrap();
    let f = BufReader::new(f);

    let mut starting_conf: Vec<Vec<u8>> = Vec::new();
    let mut current_line:Vec<u8>;

    for line in f.lines() {
        current_line = Vec::new();
        for c in line.unwrap().chars() {
            if c == '.' {
                current_line.push(0)
            } else {
                current_line.push(1);
            }
        }
        starting_conf.push(current_line);
    }
    return starting_conf;
}

fn main() {
    env_logger::init();
    debug!("{:?}", read_input_data("testData.txt"));
}
