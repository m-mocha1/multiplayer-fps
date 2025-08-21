mod fps;
mod map;
mod mechanics;
mod render;
mod sdl2;

use ::sdl2::event::Event;
use ::sdl2::keyboard::Keycode;
use ::sdl2::pixels::Color;
use ::sdl2::rect::Rect;
use std::time::Instant;

use ::sdl2::ttf;
use fps::FpsCounter;
use map::{draw_minimap_from_grid, generate_maze, maze_to_grid};
use mechanics::update_player;
use render::{Player, cast_and_draw_columns};
use sdl2::sdl2_win;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut canvas, mut event_pump) = sdl2_win("Maze FPS", 1200, 1200)?;

    let ttf_context = ttf::init()?;
    let texture_creator = canvas.texture_creator();

    let mut fps_counter = FpsCounter::new(&ttf_context, "font/Regular.ttf", 24)?;

    let maze = generate_maze(20, 15);
    let grid = maze_to_grid(&maze);

    let mut player = Player {
        x: 1.5,
        y: 1.5,
        angle: 0.0,
        fov: std::f32::consts::FRAC_PI_3,
        move_speed: 3.0,
        rot_speed: 2.5,
    };

    let mut last = Instant::now();
    let mouse_sensitivity: f32 = 0.0025;

    'game: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'game;
                }
                Event::MouseMotion { xrel, .. } => {
                    player.angle += (xrel as f32) * mouse_sensitivity;
                    use std::f32::consts::PI;
                    if player.angle > PI {
                        player.angle -= 2.0 * PI;
                    }
                    if player.angle < -PI {
                        player.angle += 2.0 * PI;
                    }
                }
                _ => {}
            }
        }

        let now = Instant::now();
        let dt = (now - last).as_secs_f32();
        last = now;

        let kbd = event_pump.keyboard_state();
        update_player(&mut player, &grid, &kbd, dt);

        // --- render world ---
        cast_and_draw_columns(&mut canvas, &grid, &player, 1200, 1200, 300)?;
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.fill_rect(Rect::new(0, 900, 1200, 300))?;
        draw_minimap_from_grid(&mut canvas, &grid, &player, 10, 8, 340, 920)?;

        // --- update + draw FPS ---
        fps_counter.update();
        fps_counter.draw(&mut canvas, &texture_creator)?;

        canvas.present();
    }

    Ok(())
}
