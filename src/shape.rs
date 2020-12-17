pub fn shape_4d(grid: &Vec<Vec<Vec<Vec<u8>>>>) -> Vec<usize> {
    return vec![grid.len(), grid[0].len(), grid[0][0].len(), grid[0][0][0].len()];
}

pub fn shape_3d(grid: &Vec<Vec<Vec<u8>>>) -> Vec<usize> {
    return vec![grid.len(), grid[0].len(), grid[0][0].len()];
}

pub fn shape_2d(grid: &Vec<Vec<u8>>) -> Vec<usize> {
    return vec![grid.len(), grid[0].len()];
}