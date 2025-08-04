use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::{Duration, Instant};

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Maze FPS", 800, 600)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let mut event_pump = sdl_context.event_pump()?;

    let mut last_frame = Instant::now();
    let mut frame_count = 0;
    let mut fps_timer = Instant::now();

    loop {
        // Event handling
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

        // Rendering
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Draw something simple (later: maze and players)
        canvas.set_draw_color(Color::RGB(255, 0, 0));
        canvas.fill_rect(sdl2::rect::Rect::new(100, 100, 50, 50))?;

        canvas.present();

        // FPS calculation
        frame_count += 1;
        let now = Instant::now();
        if now.duration_since(fps_timer).as_secs_f32() >= 1.0 {
            println!("FPS: {}", frame_count);
            frame_count = 0;
            fps_timer = now;
        }

        // Delay to cap at ~60 FPS
        let frame_time = now.duration_since(last_frame);
        if frame_time < Duration::from_millis(16) {
            std::thread::sleep(Duration::from_millis(16) - frame_time);
        }
        last_frame = now;
    }
}
