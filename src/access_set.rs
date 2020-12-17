
pub fn access_grid_3d(grid: &Vec<Vec<Vec<u8>>>, ind: &Vec<usize>) -> u8 {
    return grid[ind[0]][ind[1]][ind[2]]
}

pub fn access_grid_4d(grid: &Vec<Vec<Vec<Vec<u8>>>>, ind: &Vec<usize>) -> u8 {
    return grid[ind[0]][ind[1]][ind[2]][ind[3]]
}

pub fn set_grid_element_3d(grid: &mut Vec<Vec<Vec<u8>>>, ind: &Vec<usize>, element: u8) {
    grid[ind[0]][ind[1]][ind[2]] = element;
}

pub fn set_grid_element_4d(grid: &mut Vec<Vec<Vec<Vec<u8>>>>, ind: &Vec<usize>, element: u8) {
    grid[ind[0]][ind[1]][ind[2]][ind[3]] = element;
}