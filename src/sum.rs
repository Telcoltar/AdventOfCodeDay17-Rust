use crate::shape::{shape_3d, shape_4d};
use itertools::Itertools;
use crate::access_set::{access_grid_3d, access_grid_4d};

pub fn sum_3d(grid: Vec<Vec<Vec<u8>>>) -> i32 {
    let shape = shape_3d(&grid);
    let mut count = 0;
    let indices = shape.iter().
        map(|&dim| 0..dim).multi_cartesian_product();
    for ind in indices {
        count = count + (access_grid_3d(&grid, &ind) as i32);
    }
    return count;
}

pub fn sum_4d(grid: Vec<Vec<Vec<Vec<u8>>>>) -> i32 {
    let shape = shape_4d(&grid);
    let mut count = 0;
    let indices = shape.iter().
        map(|&dim| 0..dim).multi_cartesian_product();
    for ind in indices {
        count = count + (access_grid_4d(&grid, &ind) as i32);
    }
    return count;
}