#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Player {
    pub x: f32,
    pub y: f32,
    pub angle: f32,
    pub fov: f32,
    pub move_speed: f32,
    pub rot_speed: f32,
}
#[derive(Debug, Clone, Copy)]
pub struct OtherPlayer {
    pub x: f32,
    pub y: f32,
}

// the serrver will send the Vec of players struct to the client with their positions and angles

// random players for testing and

// this to create a simple depth-based color gradient for some effects
#[inline]
fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

#[inline]
fn lerp_rgb(n: (u8, u8, u8), f: (u8, u8, u8), t: f32) -> (u8, u8, u8) {
    let t = t.clamp(0.0, 1.0);
    (
        lerp(n.0 as f32, f.0 as f32, t).round() as u8,
        lerp(n.1 as f32, f.1 as f32, t).round() as u8,
        lerp(n.2 as f32, f.2 as f32, t).round() as u8,
    )
}

/// Nonlinear remap so the gradient stays darker near you and brightens smoothly.
/// k ~ 1.5..3.0 looks nice. Adjust to taste.
#[inline]
fn depth_curve(t: f32, k: f32) -> f32 {
    // Option A: simple power curve
    t.clamp(0.0, 1.0).powf(k)
    // Option B: 1/(1 + a/t) type can also be used; stick to one curve for consistency.
}

pub fn cast_and_draw_columns(
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
    grid: &Vec<Vec<u8>>,    // 0 = empty, 1 = wall
    p: &Player,             // needs x, y, angle, fov
    others: &[OtherPlayer], // NEW: slice of other players with (x, y)
    screen_w: i32,
    screen_h: i32, // full window height
    reserved: i32, // reserved pixels at the bottom (e.g. for minimap)
) -> Result<(), String> {
    use sdl2::pixels::Color;

    // z-buffer for sprite rendering: distance to wall for each vertical stripe
    let mut zbuffer = vec![f32::INFINITY; screen_w as usize];

    // visible area = total height minus reserved area
    let view_h = screen_h - reserved;
    let half = view_h / 2;

    // --- draw ceiling & floor gradients (unchanged) ---
    let (ceil_near, ceil_far, floor_near, floor_far) = (
        (18, 18, 20), // ceiling close
        (8, 9, 12),   // ceiling horizon
        (50, 50, 54), // floor close
        (20, 20, 24), // floor far
    );

    for y in 0..half {
        let t = 1.0 - (y as f32 / half as f32);
        let t = depth_curve(t, 1.6);
        let (r, g, b) = lerp_rgb(ceil_far, ceil_near, t);
        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.draw_line((0, y), (screen_w - 1, y))?;
    }

    for y in half..view_h {
        let raw = (y - half) as f32 / half as f32;
        let t = depth_curve(raw, 2.4);
        let (r0, g0, b0) = lerp_rgb(floor_far, floor_near, t);
        let dark_mul = 0.85;
        let r = (r0 as f32 * dark_mul) as u8;
        let g = (g0 as f32 * dark_mul) as u8;
        let b = (b0 as f32 * dark_mul) as u8;

        canvas.set_draw_color(Color::RGB(r, g, b));
        canvas.draw_line((0, y), (screen_w - 1, y))?;
    }

    // --- camera setup ---
    let dir_x = p.angle.cos();
    let dir_y = p.angle.sin();
    let plane_scale = (p.fov * 0.5).tan();
    let plane_x = -dir_y * plane_scale;
    let plane_y = dir_x * plane_scale;

    // --- WALLS + fill zbuffer ---
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

        // FILL zbuffer for this column (NEW: 1 line)
        zbuffer[x as usize] = perp_dist;

        let base = if side == 1 {
            (68, 68, 68)
        } else {
            (43, 43, 43)
        };
        let fog_k = 0.50_f32;
        let min_brightness = 0.30_f32;
        let dist_mul = (1.0 / (1.0 + fog_k * perp_dist)).clamp(min_brightness, 1.0);
        let (r, g, b) = shade_rgb(base, dist_mul);

        if perp_dist.is_finite() && perp_dist > 0.0001 {
            let line_h = (view_h as f32 / perp_dist) as i32;
            let draw_start = (view_h - line_h) / 2;
            let draw_end = (view_h + line_h) / 2;

            canvas.set_draw_color(Color::RGB(r, g, b));

            let ys = draw_start.max(0);
            let ye = draw_end.min(view_h - 1);
            if ys <= ye {
                canvas.draw_line((x, ys), (x, ye))?;
            }
        }
    }

    // --- OTHER PLAYERS (render only when visible) ---
    // Camera-space transform constants
    let inv_det = {
        let det = plane_x * dir_y - dir_x * plane_y;
        if det.abs() < 1e-6 { 1e6 } else { 1.0 / det }
    };

    for other in others {
        // relative position
        let dx = other.x - p.x;
        let dy = other.y - p.y;

        // camera space
        let transform_x = inv_det * (dir_y * dx - dir_x * dy);
        let transform_y = inv_det * (-plane_y * dx + plane_x * dy);

        // behind camera? skip
        if transform_y <= 0.0 {
            continue;
        }

        // screen x
        let sprite_screen_x = ((screen_w as f32 / 2.0) * (1.0 + transform_x / transform_y)) as i32;

        // size by distance
        let sprite_h = (view_h as f32 / transform_y) as i32;
        let sprite_w = sprite_h;

        // vertical span (clamped to view)
        let mut draw_start_y = (view_h / 2) - (sprite_h / 2);
        let mut draw_end_y = draw_start_y + sprite_h;
        if draw_start_y < 0 {
            draw_start_y = 0;
        }
        if draw_end_y > view_h {
            draw_end_y = view_h;
        }

        // horizontal span (clamped to screen)
        let mut draw_start_x = sprite_screen_x - (sprite_w / 2);
        let mut draw_end_x = draw_start_x + sprite_w;
        if draw_start_x < 0 {
            draw_start_x = 0;
        }
        if draw_end_x > screen_w {
            draw_end_x = screen_w;
        }

        // column-by-column with depth test against walls
        canvas.set_draw_color(Color::RGB(0, 200, 0)); // simple green box/player
        for stripe in draw_start_x..draw_end_x {
            // sprite depth (transform_y) must be < wall depth at this column
            if transform_y < zbuffer[stripe as usize] {
                let h = (draw_end_y - draw_start_y).max(1) as u32;
                let _ = canvas.fill_rect(sdl2::rect::Rect::new(stripe, draw_start_y, 1, h));
            }
        }
    }

    Ok(())
}

// Simple shading function to darken a color by a multiplier (0.0 to 1.0)
fn shade_rgb((r, g, b): (u8, u8, u8), mul: f32) -> (u8, u8, u8) {
    let m = mul.clamp(0.0, 1.0);
    (
        (r as f32 * m).round().clamp(0.0, 255.0) as u8,
        (g as f32 * m).round().clamp(0.0, 255.0) as u8,
        (b as f32 * m).round().clamp(0.0, 255.0) as u8,
    )
}
