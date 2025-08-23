// src/mechanics.rs
use crate::render::Player;
use sdl2::keyboard::{KeyboardState, Scancode};

const MOVE_SPEED: f32 = 1.5; // units / second
const ROT_SPEED: f32 = 1.0; // radians / second  it's the mouse sensitivity
const RADIUS: f32 = 0.20; // player collision radius (tile = 1.0)  detect collision with walls 

pub fn update_player(player: &mut Player, grid: &[Vec<u8>], kbd: &KeyboardState, mut dt: f32) {
    // acceleration Speeds are per-second; scale by dt to enxure uniform movement speed
    // acroos different frame rates
    let move_speed = MOVE_SPEED * dt;
    let rot_speed = ROT_SPEED * dt;

    // Rotate / mouse look
    if kbd.is_scancode_pressed(Scancode::Left) {
        player.angle -= rot_speed;
    }
    if kbd.is_scancode_pressed(Scancode::Right) {
        player.angle += rot_speed;
    }

    // Direction vectors
    let dir_x = player.angle.cos();
    let dir_y = player.angle.sin();
    let side_x = -dir_y;
    let side_y = dir_x;

    // Input → desired movement vector
    let mut mv_x = 0.0;
    let mut mv_y = 0.0;

    if kbd.is_scancode_pressed(Scancode::W) {
        mv_x += dir_x * move_speed;
        mv_y += dir_y * move_speed;
    }
    if kbd.is_scancode_pressed(Scancode::S) {
        mv_x -= dir_x * move_speed;
        mv_y -= dir_y * move_speed;
    }
    if kbd.is_scancode_pressed(Scancode::D) {
        mv_x += side_x * move_speed;
        mv_y += side_y * move_speed;
    }
    if kbd.is_scancode_pressed(Scancode::A) {
        mv_x -= side_x * move_speed;
        mv_y -= side_y * move_speed;
    }

    // if the player is not moving, skip the skip collision checks which is the rest of the function
    if mv_x == 0.0 && mv_y == 0.0 {
        return;
    }

    let (mut nx, mut ny) = (player.x + mv_x, player.y + mv_y); // newest position if no collision

    // ---- Collision with radius + axis separation (allows sliding on walls with no sticking ) ----
    // helper to test if a circle at (x,y) with radius R intersects any wall cell
    let can_stand = |x: f32, y: f32, grid: &[Vec<u8>]| -> bool {
        // check the 3x3 neighborhood around the player
        let xi = x.floor() as i32;
        let yi = y.floor() as i32;
        for gy in (yi - 1)..=(yi + 1) {
            for gx in (xi - 1)..=(xi + 1) {
                if gy < 0 || gx < 0 {
                    continue;
                }
                let (gyu, gxu) = (gy as usize, gx as usize);
                if gyu >= grid.len() || gxu >= grid[0].len() {
                    continue;
                }
                if grid[gyu][gxu] == 0 {
                    continue;
                } // not a wall
                // axis-aligned box of the wall cell
                let wx0 = gxu as f32;
                let wy0 = gyu as f32;
                let wx1 = wx0 + 1.0;
                let wy1 = wy0 + 1.0;

                // closest point on the wall box to (x,y)
                let cx = x.clamp(wx0, wx1);
                let cy = y.clamp(wy0, wy1);
                let dx = x - cx;
                let dy = y - cy;
                if dx * dx + dy * dy < RADIUS * RADIUS {
                    return false; // overlap
                }
            }
        }
        true
    };

    // try full move
    if can_stand(nx, ny, grid) {
        player.x = nx;
        player.y = ny;
        return;
    }
    // try X-only (slide along wall)
    nx = player.x + mv_x;
    if can_stand(nx, player.y, grid) {
        player.x = nx;
    }
    // try Y-only
    ny = player.y + mv_y;
    if can_stand(player.x, ny, grid) {
        player.y = ny;
    }
}
