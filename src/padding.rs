use crate::shape::{shape_4d, shape_3d, shape_2d};
use log::{debug};

pub fn padding_4d_array(grid: Vec<Vec<Vec<Vec<u8>>>>) -> Vec<Vec<Vec<Vec<u8>>>> {
    let shape = shape_4d(&grid);
    debug!("{:?}", shape);
    let mut padded_grid: Vec<Vec<Vec<Vec<u8>>>> = Vec::new();
    padded_grid.push(vec![vec![vec![0; shape[3] + 2]; shape[2] + 2]; shape[1] + 2]);
    let shape = shape_4d(&padded_grid);
    debug!("{:?}", shape);
    for sub_grid in grid {
        padded_grid.push(padding_3d_array(sub_grid));
    }
    padded_grid.push(vec![vec![vec![0; shape[3] + 2]; shape[2] + 2]; shape[1] + 2]);
    return padded_grid;
}

pub fn padding_3d_array(grid: Vec<Vec<Vec<u8>>>) -> Vec<Vec<Vec<u8>>> {
    let shape = shape_3d(&grid);
    let mut padded_grid: Vec<Vec<Vec<u8>>> = Vec::new();
    padded_grid.push(vec![vec![0; shape[2] + 2]; shape[1] + 2]);
    for sub_grid in grid {
        padded_grid.push(padding_2d_array(sub_grid));
    }
    padded_grid.push(vec![vec![0; shape[2] + 2]; shape[1] + 2]);
    return padded_grid;
}

pub fn padding_2d_array(grid: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let shape = shape_2d(&grid);
    let mut padded_grid: Vec<Vec<u8>> = Vec::new();
    padded_grid.push(vec![0; shape[1] + 2]);
    // let mut line: Vec<u8>;
    for mut line in grid {
        // line = grid.remove(0);
        line.insert(0,0);
        line.push(0);
        padded_grid.push(line);
    }
    padded_grid.push(vec![0; shape[1] + 2]);
    return padded_grid;
}