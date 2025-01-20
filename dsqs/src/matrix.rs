//Given a 2D grid of size m x n and an integer k. You need to shift the grid k times.
/**In one shift operation:

    Element at grid[i][j] moves to grid[i][j + 1].
    Element at grid[i][n - 1] moves to grid[i + 1][0].
    Element at grid[m - 1][n - 1] moves to grid[0][0].

Return the 2D grid after applying shift operation k times. */

pub fn shift_grid(grid: Vec<Vec<i32>>, k: usize) -> Vec<Vec<i32>> {
    let m = grid.len();
    let n = grid[0].len(); 
    let total_elements = m * n;
    
    let mut flat_grid: Vec<i32> = grid.into_iter().flatten().collect();
    
    let k = k % total_elements;
    flat_grid.rotate_right(k); 
    flat_grid.chunks(n).map(|chunk| chunk.to_vec()).collect()
}
 