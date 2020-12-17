mod test_main;
mod padding;
mod shape;
mod sum;
mod access_set;

use std::fs::File;
use std::io::{BufReader, BufRead};
use log::{debug, info};
use itertools::Itertools;
use padding::padding_3d_array;
use crate::shape::{shape_3d, shape_4d};
use crate::access_set::{access_grid_3d, set_grid_element_3d, access_grid_4d, set_grid_element_4d};
use crate::sum::{sum_3d, sum_4d};
use crate::padding::padding_4d_array;

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

fn to_3d_array(grid: Vec<Vec<u8>>) -> Vec<Vec<Vec<u8>>> {
    return vec![grid];
}

fn to_4d_array(grid: Vec<Vec<u8>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    let grid_3d = to_3d_array(grid);
    debug!("{:?}", shape_3d(&grid_3d));
    let grid_4d = vec![grid_3d];
    debug!("{:?}", shape_4d(&grid_4d));
    return grid_4d;
}

fn print_grid_3d(grid: &Vec<Vec<Vec<u8>>>) -> String {
    let mut repr: String = String::new();
    repr.push_str("\n");
    for sub_grid in grid {
        for sub_sub_grid in sub_grid {
            repr.push_str(&format!("{:?}", sub_sub_grid));
            repr.push_str("\n");
        }
        repr.push_str("\n");
    }
    return repr;
}

fn get_active_neighbors_3d(grid: &Vec<Vec<Vec<u8>>>, origin: Vec<usize>) -> u8 {
    let mut count = 0;
    let indices = origin.iter().
        map(|coord| (coord-1)..(coord + 2)).multi_cartesian_product();
    for ind in indices {
        if ind == origin {
            continue;
        } else {
            count += access_grid_3d(&grid, &ind);
        }
    }
    return count;
}

fn get_active_neighbors_4d(grid: &Vec<Vec<Vec<Vec<u8>>>>, origin: Vec<usize>) -> u8 {
    let mut count = 0;
    let indices = origin.iter().
        map(|coord| (coord-1)..(coord + 2)).multi_cartesian_product();
    for ind in indices {
        if ind == origin {
            continue;
        } else {
            count += access_grid_4d(&grid, &ind);
        }
    }
    return count;
}

fn cycle_3d(grid: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    let mut padded_grid = padding_3d_array(grid);
    let const_padded_grid = padded_grid.clone();

    let shape = shape_3d(&const_padded_grid);
    let indices = shape.iter().
        map(|dim| 1..(dim - 1)).multi_cartesian_product();
    let mut active_neighbors: u8;
    for ind in indices {
        if access_grid_3d(&const_padded_grid, &ind) == 1 {
            active_neighbors = get_active_neighbors_3d(&const_padded_grid, ind.clone());
            if (2 > active_neighbors) || (active_neighbors > 3) {
                set_grid_element_3d(&mut padded_grid, &ind,  0);
            }
        } else {
            active_neighbors = get_active_neighbors_3d(&const_padded_grid, ind.clone());
            if active_neighbors == 3 {
                set_grid_element_3d(&mut padded_grid, &ind,  1);
            }
        }
    }
    return padded_grid;
}

fn cycle_4d(grid: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    let mut padded_grid = padding_4d_array(grid);
    let const_padded_grid = padded_grid.clone();

    let shape = shape_4d(&const_padded_grid);
    debug!("{:?}", shape);
    let indices = shape.iter().
        map(|dim| 1..(dim - 1)).multi_cartesian_product();
    let mut active_neighbors: u8;
    for ind in indices {
        if access_grid_4d(&const_padded_grid, &ind) == 1 {
            active_neighbors = get_active_neighbors_4d(&const_padded_grid, ind.clone());
            if (2 > active_neighbors) || (active_neighbors > 3) {
                set_grid_element_4d(&mut padded_grid, &ind,  0);
            }
        } else {
            active_neighbors = get_active_neighbors_4d(&const_padded_grid, ind.clone());
            if active_neighbors == 3 {
                set_grid_element_4d(&mut padded_grid, &ind,  1);
            }
        }
    }
    return padded_grid;
}

fn solution_part_1(file_name: &str) -> i32 {
    let mut grid = padding_3d_array(to_3d_array(
        read_input_data(file_name)));
    debug!("{}", print_grid_3d(&grid));
    for _i in 0..6 {
        grid = cycle_3d(grid);
    }
    return sum_3d(grid);
}

fn solution_part_2(file_name: &str) -> i32 {
    let mut grid = padding_4d_array(to_4d_array(
        read_input_data(file_name)));
    let shape = shape_4d(&grid);
    debug!("{:?}", shape);
    for _i in 0..6 {
        grid = cycle_4d(grid);
    }
    return sum_4d(grid);
}

fn main() {
    env_logger::init();
    info!("{}", solution_part_1("inputData.txt"));
    info!("{}", solution_part_2("inputData.txt"));
}
