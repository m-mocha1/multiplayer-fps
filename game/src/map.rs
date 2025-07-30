use rand::seq::SliceRandom;
use rand::thread_rng;

#[derive(Clone, Copy)]
struct Cell {
    visited: bool,
    walls: [bool; 4], // top, right, bottom, left
}

impl Cell {
    fn new() -> Self {
        Self {
            visited: false,
            walls: [true; 4],
        }
    }
}

fn generate_maze(w: usize, h: usize) -> Vec<Vec<Cell>> {
    let mut grid = vec![vec![Cell::new(); w]; h];
    backtrack(0, 0, w, h, &mut grid);
    grid
}

fn backtrack(x: usize, y: usize, w: usize, h: usize, grid: &mut Vec<Vec<Cell>>) {
    grid[y][x].visited = true;

    let mut dirs = vec![(0, -1, 0, 2), (1, 0, 1, 3), (0, 1, 2, 0), (-1, 0, 3, 1)];
    dirs.shuffle(&mut thread_rng());

    for &(dx, dy, wall, opp_wall) in &dirs {
        let nx = x as isize + dx;
        let ny = y as isize + dy;

        if nx >= 0 && ny >= 0 && (nx as usize) < w && (ny as usize) < h {
            let (nx, ny) = (nx as usize, ny as usize);
            if !grid[ny][nx].visited {
                grid[y][x].walls[wall] = false;
                grid[ny][nx].walls[opp_wall] = false;
                backtrack(nx, ny, w, h, grid);
            }
        }
    }
}
