use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use std::time::{Duration, Instant};

pub fn sdl2_win(
    title: &str,
    width: u32,
    height: u32,
) -> Result<(Canvas<Window>, sdl2::EventPump), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window(title, width, height)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let event_pump = sdl_context.event_pump()?;

    sdl_context.mouse().set_relative_mouse_mode(true);
    
    Ok((canvas, event_pump))
}
