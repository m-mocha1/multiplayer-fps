use sdl2::pixels::Color;
use sdl2::rect::Rect;
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
    pub move_speed: f32,
    pub rot_speed: f32,
}

pub fn cast_and_draw_columns(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    grid: &Vec<Vec<u8>>, // 0 = empty, 1 = wall
    p: &Player,
    screen_w: i32,
    screen_h: i32, // full window height
    reserved: i32, // reserved pixels at the bottom (e.g. for minimap)
) -> Result<(), String> {
    // visible area = total height minus reserved area
    let view_h = screen_h - reserved;

    // --- draw ceiling ---
    canvas.set_draw_color(Color::RGB(12, 12, 16));
    canvas.fill_rect(Rect::new(0, 0, screen_w as u32, (view_h / 2) as u32))?;

    // --- draw floor ---
    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.fill_rect(Rect::new(
        0,
        view_h / 2,
        screen_w as u32,
        (view_h / 2) as u32,
    ))?;

    // --- camera setup ---
    let dir_x = p.angle.cos();
    let dir_y = p.angle.sin();
    let plane_scale = (p.fov * 0.5).tan();
    let plane_x = -dir_y * plane_scale;
    let plane_y = dir_x * plane_scale;

    for x in 0..screen_w {
        let camera_x = 2.0 * (x as f32) / (screen_w as f32) - 1.0;
        let ray_dir_x = dir_x + plane_x * camera_x;
        let ray_dir_y = dir_y + plane_y * camera_x;

        let mut map_x = p.x.floor() as i32;
        let mut map_y = p.y.floor() as i32;

        let delta_dist_x = if ray_dir_x == 0.0 {
            f32::INFINITY
        } else {
            (1.0 / ray_dir_x).abs()
        };
        let delta_dist_y = if ray_dir_y == 0.0 {
            f32::INFINITY
        } else {
            (1.0 / ray_dir_y).abs()
        };

        let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
            (-1, (p.x - map_x as f32) * delta_dist_x)
        } else {
            (1, ((map_x as f32 + 1.0) - p.x) * delta_dist_x)
        };
        let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
            (-1, (p.y - map_y as f32) * delta_dist_y)
        } else {
            (1, ((map_y as f32 + 1.0) - p.y) * delta_dist_y)
        };

        let mut hit = false;
        let mut side = 0;
        while !hit {
            if side_dist_x < side_dist_y {
                side_dist_x += delta_dist_x;
                map_x += step_x;
                side = 0;
            } else {
                side_dist_y += delta_dist_y;
                map_y += step_y;
                side = 1;
            }
            if map_y < 0
                || map_y as usize >= grid.len()
                || map_x < 0
                || map_x as usize >= grid[0].len()
            {
                break;
            }
            if grid[map_y as usize][map_x as usize] != 0 {
                hit = true;
            }
        }

        let perp_dist = if side == 0 {
            (map_x as f32 - p.x + (1 - step_x) as f32 * 0.5) / ray_dir_x
        } else {
            (map_y as f32 - p.y + (1 - step_y) as f32 * 0.5) / ray_dir_y
        }
        .abs();

        if perp_dist.is_finite() && perp_dist > 0.0001 {
            let line_h = (view_h as f32 / perp_dist) as i32;
            let draw_start = (view_h - line_h) / 2;
            let draw_end = (view_h + line_h) / 2;
            let mut  r:u8 = 0;
            let mut g :u8= 0;
            let  mut b: u8 = 0;
           if side == 1 { r = 20;g = 52; b = 164 } else { r = 0;g = 0; b = 139 };

            canvas.set_draw_color(Color::RGB(r, g, b));
            canvas.draw_line((x, draw_start), (x, draw_end))?;
        }
    }

    Ok(())
}
