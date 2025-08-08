use rand::seq::SliceRandom;
use rand::thread_rng;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

#[derive(Debug, Clone, Copy)]
pub struct Cell {
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

pub fn generate_maze(w: usize, h: usize) -> Vec<Vec<Cell>> {
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
pub fn draw_maze(
    canvas: &mut Canvas<Window>,
    maze: &Vec<Vec<Cell>>,
    cell_size: i32,
    offset_x: i32,
    offset_y: i32,
) -> Result<(), String> {
    let rows = maze.len();
    let cols = maze[0].len();

    for y in 0..rows {
        for x in 0..cols {
            let cell = maze[y][x];
            let x1 = offset_x + (x as i32) * cell_size;
            let y1 = offset_y + (y as i32) * cell_size;
            let x2 = x1 + cell_size;
            let y2 = y1 + cell_size;

            // Set wall color
            canvas.set_draw_color(Color::RGB(255, 255, 255));

            if cell.walls[0] {
                canvas.draw_line((x1, y1), (x2, y1))?; // top
            }
            if cell.walls[1] {
                canvas.draw_line((x2, y1), (x2, y2))?; // right
            }
            if cell.walls[2] {
                canvas.draw_line((x1, y2), (x2, y2))?; // bottom
            }
            if cell.walls[3] {
                canvas.draw_line((x1, y1), (x1, y2))?; // left
            }
        }
    }

    Ok(())
}
