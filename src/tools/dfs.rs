// Depth-first traversal
//
// 1. [x] Pass the grid and starting cordinates
// 2. [x] Create an empty grid with the same coordinates as the given grid
// 3. [ ] Push the starting coordinates into a vector
// 4. [ ] While the stack is full (vector not emopty):
//  5. [ ] Pop the top coordinates off the stack/vector (x, y)
//  6. [ ] Check if x and y are within the bounds of the grid and not visited
//      - If so, mark empty_grid[x][y] as True
//  7. [ ] Look at the neighbors in all 4 directions
//      - If any are the next possible number (ie 1 more than current pos):
//      - Create a variable that represents one position in the direction of that number 
//      - Check if this cell is within the bounds of the grid and not visited
//      - If criteria are met, push the next number onto the stack



// TODO: Generalize for T
fn initialize_empty_grid(len_x: usize, len_y: usize) {
    let empty_grid: Vec<Vec<i8>> = Vec::with_capacity(x);

    for x in 0..len_x {
        let row: Vec<i8> = Vec::new();
        for y in 0..len_y {
            row.push(-1);
        }
        empty_grid.push(row);
    }
}

fn within_grid(grid: &Vec<Vec<i8>>, x: usize, y: usize) {
    (x < grid.len() && x >= 0) &&
    (y < grid[0].len() && y >= 0)
}

const CARDINALS: [(usize, usize); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn find_all_paths(
    grid: &Vec<Vec<i8>,
    start_x: usize,
    start_y: usize
) -> Vec<Vec<(usize, usize)>> {
    let visited: Vec<Vec<bool>> = initialize_empty_grid(
        grid.len(),
        grid[0].len()
    );
    let stack: Vec<(usize, usize)> = Vec::new();
    let collected_trails: Vec<Vec<(usize, usize)>> = Vec::new();

    stack.push((start_x, start_y));

    while !stack.is_empty() {
        let (x, y) = stack.pop();

        if within_grid(grid, x, y) && !visited[x][y] {
            visited[x][y] = true;

            for (dx, dy) in CARDINALS {
                let next_x = x + dx;
                let next_y = y + dx;
                
                if within_grid(grid, x, y) && !visited[next_x][next_y] {
                    if grid[next_x][next_y] == grid[x][y] + 1 {
                        stack.push((next_x, next_y));
                        if grid[next_x][next_y] == 9 {
                            collected_trails.push(stack);
                        }
                    }
                }
            }
        }
    }

    collected_trails
}