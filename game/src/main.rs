mod map;
mod sdl2;
use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use ::sdl2::pixels::Color;
use map::{draw_maze, generate_maze};
use sdl2::sdl2_win;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut canvas, mut event_pump) = sdl2_win("Maze FPS", 800, 600)?;
    let maze = generate_maze(20, 15); // 20x15 rectangle maze
    let cell_size = 30;

    loop {
        for event in event_pump.poll_iter() {
            if let Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } = event
            {
                return Ok(());
            }
        }

        // Calculate offsets to center the maze to make center the widnow for now
        let maze_width_px = maze[0].len() as i32 * cell_size;
        let maze_height_px = maze.len() as i32 * cell_size;
        let offset_x = (800 - maze_width_px) / 2;
        let offset_y = (600 - maze_height_px) / 2;

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        draw_maze(&mut canvas, &maze, cell_size, offset_x, offset_y)?;

        canvas.present();
        std::thread::sleep(std::time::Duration::from_millis(16)); // 60 FPS
    }
}
