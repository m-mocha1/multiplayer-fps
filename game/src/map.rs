use rand::prelude::*;
use rand::rng;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Cell {
    pub visited: bool,
    pub walls: [bool; 4],
}

pub fn make_map(width: usize, height: usize) -> Vec<Vec<Cell>> {
    // Create a 2d grid of (w * h) where the cell is not visited and all 4 walls are up.
    let mut grid = vec![
        vec![
            Cell {
                visited: false,
                walls: [true; 4],
            };
            width
        ];
        height
    ];

    let mut stack = vec![(0, 0)];
    grid[0][0].visited = true; // start from top left cell
    //                                                   dx   dy   wall of the cell    wall of the next cell
    let directions = [(0, -1, 0, 2), (1, 0, 1, 3), (0, 1, 2, 0), (-1, 0, 3, 1)];

    // stack returns x,y if the stack is not empty
    while let Some((x, y)) = stack.pop() {
        let mut neighbors = vec![];

        for &(dx, dy, wall, opp_wall) in &directions {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && ny >= 0 && nx < width as isize && ny < height as isize {
                let nx = nx as usize;
                let ny = ny as usize;
                if !grid[ny][nx].visited {
                    neighbors.push((nx, ny, wall, opp_wall));
                }
            }
        }

        if !neighbors.is_empty() {
            stack.push((x, y));

            let mut rng = rng();
            let &(nx, ny, wall, opp_wall) = neighbors.choose(&mut rng).unwrap();
            grid[y][x].walls[wall] = false;
            grid[ny][nx].walls[opp_wall] = false;
            grid[ny][nx].visited = true;
            stack.push((nx, ny));
        }
    }
    grid
}
