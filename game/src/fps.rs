use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::TextureQuery;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;

use std::time::Instant;

pub struct FpsCounter<'a> {
    font: Font<'a, 'a>,
    last_frame: Instant,
    fps: f32,
}

impl<'a> FpsCounter<'a> {
    pub fn new(
        ttf_context: &'a Sdl2TtfContext,
        font_path: &str,
        size: u16,
    ) -> Result<Self, String> {
        let font = ttf_context.load_font(font_path, size)?;
        Ok(Self {
            font,
            last_frame: Instant::now(),
            fps: 0.0,
        })
    }

    pub fn update(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;

        self.fps = 1.0 / delta.as_secs_f32();
    }

    pub fn draw(
        &self,
        canvas: &mut Canvas<Window>,
        texture_creator: &TextureCreator<sdl2::video::WindowContext>,
    ) -> Result<(), String> {
        let text = format!("FPS: {:.1}", self.fps);
        let surface = self
            .font
            .render(&text)
            .blended(Color::RGB(255, 255, 255))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let TextureQuery { width, height, .. } = texture.query();
        let target = Rect::new(20, 20, width, height);

        canvas.copy(&texture, None, Some(target))?;
        Ok(())
    }
}
