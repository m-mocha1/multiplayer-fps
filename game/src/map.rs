use crate::render::Player;
use rand::seq::SliceRandom;
use rand::thread_rng;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
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


pub fn maze_to_grid(maze: &Vec<Vec<Cell>>) -> Vec<Vec<u8>> {
    let h = maze.len();
    let w = maze[0].len();
    let gh = h * 2 + 1;
    let gw = w * 2 + 1;
    let mut grid = vec![vec![1u8; gw]; gh];

    for cy in 0..h {
        for cx in 0..w {
            let gx = 2 * cx + 1;
            let gy = 2 * cy + 1;
            grid[gy][gx] = 0; // floor in the middle of each maze cell

            // walls: [top, right, bottom, left]
            let cell = maze[cy][cx];

            // open passage if there is NO wall
            if !cell.walls[0] {
                grid[gy - 1][gx] = 0;
            } // top
            if !cell.walls[1] {
                grid[gy][gx + 1] = 0;
            } // right
            if !cell.walls[2] {
                grid[gy + 1][gx] = 0;
            } // bottom
            if !cell.walls[3] {
                grid[gy][gx - 1] = 0;
            } // left
        }
    }

    grid
}
pub fn draw_minimap_from_grid(
    canvas: &mut Canvas<Window>,
    grid: &[Vec<u8>],
    player: &Player,
    scale: i32,
    ox: i32,
    oy: i32,
) -> Result<(), String> {
    // walls
    for (gy, row) in grid.iter().enumerate() {
        for (gx, &cell) in row.iter().enumerate() {
            if cell != 0 {
                let x = ox + (gx as i32) * scale;
                let y = oy + (gy as i32) * scale;
                canvas.set_draw_color(Color::RGB(0, 204, 204));
                canvas.fill_rect(Rect::new(x, y, scale as u32, scale as u32))?;
            }
        }
    }

    // player marker (red square)
    let px = ox + (player.x * scale as f32) as i32;
    let py = oy + (player.y * scale as f32) as i32;
    canvas.set_draw_color(Color::RGB(255, 0, 0));
    canvas.fill_rect(Rect::new(px - 2, py - 2, 4, 4))?;

    // facing direction (yellow line)
    let look_len = (6 * scale).max(8) as f32;
    let lx = px as f32 + player.angle.cos() * look_len;
    let ly = py as f32 + player.angle.sin() * look_len;
    canvas.set_draw_color(Color::RGB(255, 255, 0));
    canvas.draw_line((px, py), (lx as i32, ly as i32))?;

    Ok(())
}
